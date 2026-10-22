// Kvant (kv) — Micro terminal text editor in pure Rust with full keyboard navigation
// Zero external dependencies — pure standard library + POSIX libc FFI
// Standard: IEEE Std 1003.1 (POSIX) / Gosplan Minimalist Architecture

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process;

// -----------------------------------------------------------------------------
// POSIX Termios and ioctl FFI Definitions
// -----------------------------------------------------------------------------
const TIOCGWINSZ: u64 = 0x5413;
const TCSAFLUSH: i32 = 2;

const BRKINT: u32 = 2;
const ICRNL: u32 = 256;
const INPCK: u32 = 16;
const ISTRIP: u32 = 32;
const IXON: u32 = 1024;
const OPOST: u32 = 1;
const CS8: u32 = 48;
const ECHO: u32 = 8;
const ICANON: u32 = 2;
const IEXTEN: u32 = 32768;
const ISIG: u32 = 1;
const VMIN: usize = 6;
const VTIME: usize = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

extern "C" {
    fn tcgetattr(fd: i32, termios_p: *mut Termios) -> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const Termios) -> i32;
    fn ioctl(fd: i32, request: u64, ...) -> i32;
}

// -----------------------------------------------------------------------------
// Terminal Raw Mode RAII Guard
// -----------------------------------------------------------------------------
pub struct RawModeGuard {
    orig_termios: Termios,
    active: bool,
}

impl RawModeGuard {
    pub fn enter() -> io::Result<Self> {
        let mut orig = Termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };

        unsafe {
            if tcgetattr(0, &mut orig) != 0 {
                return Err(io::Error::last_os_error());
            }
        }

        let mut raw = orig;
        raw.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
        raw.c_oflag &= !OPOST;
        raw.c_cflag |= CS8;
        raw.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
        raw.c_cc[VMIN] = 0; // Non-blocking with timeout
        raw.c_cc[VTIME] = 1; // 100ms timeout for escape sequences

        unsafe {
            if tcsetattr(0, TCSAFLUSH, &raw) != 0 {
                return Err(io::Error::last_os_error());
            }
        }

        // Switch to alternate screen and clear
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[?1049h\x1b[?25l")?;
        stdout.flush()?;

        Ok(Self {
            orig_termios: orig,
            active: true,
        })
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        if self.active {
            let mut stdout = io::stdout();
            // Show cursor, leave alternate screen
            let _ = write!(stdout, "\x1b[?25h\x1b[?1049l");
            let _ = stdout.flush();

            unsafe {
                tcsetattr(0, TCSAFLUSH, &self.orig_termios);
            }
            self.active = false;
        }
    }
}

// -----------------------------------------------------------------------------
// Key Event Representation
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Ctrl(char),
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Backspace,
    Enter,
    Tab,
    Escape,
    None,
}

pub fn read_key() -> Key {
    let mut stdin = io::stdin();
    let mut buf = [0u8; 1];

    if stdin.read(&mut buf).unwrap_or(0) == 0 {
        return Key::None;
    }

    match buf[0] {
        b'\x1b' => {
            let mut seq = [0u8; 3];
            if stdin.read(&mut seq[0..1]).unwrap_or(0) == 0 {
                return Key::Escape;
            }

            if seq[0] == b'[' || seq[0] == b'O' {
                if stdin.read(&mut seq[1..2]).unwrap_or(0) == 0 {
                    return Key::Escape;
                }

                if seq[1] >= b'0' && seq[1] <= b'9' {
                    if stdin.read(&mut seq[2..3]).unwrap_or(0) == 0 {
                        return Key::Escape;
                    }
                    if seq[2] == b'~' {
                        return match seq[1] {
                            b'1' | b'7' => Key::Home,
                            b'3' => Key::Delete,
                            b'4' | b'8' => Key::End,
                            b'5' => Key::PageUp,
                            b'6' => Key::PageDown,
                            _ => Key::Escape,
                        };
                    }
                } else {
                    return match seq[1] {
                        b'A' => Key::Up,
                        b'B' => Key::Down,
                        b'C' => Key::Right,
                        b'D' => Key::Left,
                        b'H' => Key::Home,
                        b'F' => Key::End,
                        _ => Key::Escape,
                    };
                }
            }
            Key::Escape
        }
        b'\r' | b'\n' => Key::Enter,
        b'\t' => Key::Tab,
        127 | 8 => Key::Backspace,
        1..=26 => {
            let c = (b'a' + buf[0] - 1) as char;
            Key::Ctrl(c)
        }
        c => Key::Char(c as char),
    }
}

// -----------------------------------------------------------------------------
// Terminal Size Detection
// -----------------------------------------------------------------------------
pub fn get_terminal_size() -> (usize, usize) {
    let mut ws = Winsize::default();
    let res = unsafe { ioctl(1, TIOCGWINSZ, &mut ws) };
    if res == 0 && ws.ws_col > 0 && ws.ws_row > 0 {
        (ws.ws_col as usize, ws.ws_row as usize)
    } else {
        (80, 24)
    }
}

// -----------------------------------------------------------------------------
// Editor Core State
// -----------------------------------------------------------------------------
pub struct Editor {
    pub rows: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub row_offset: usize,
    pub col_offset: usize,
    pub screen_cols: usize,
    pub screen_rows: usize,
    pub filename: Option<String>,
    pub dirty: bool,
    pub status_msg: String,
    pub quit_confirmation: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuitDecision {
    SaveAndQuit,
    DiscardAndQuit,
    Cancel,
}

impl Editor {
    pub fn confirm_quit_action<F>(&mut self, mut key_reader: F) -> QuitDecision
    where
        F: FnMut() -> Key,
    {
        if !self.dirty {
            return QuitDecision::DiscardAndQuit;
        }

        self.status_msg =
            String::from("⚠️ Alterações não salvas! [S]alvar e sair | [D]escartar | [C]ancelar");
        let _ = self.render();

        loop {
            match key_reader() {
                Key::Char('s') | Key::Char('S') => return QuitDecision::SaveAndQuit,
                Key::Char('d') | Key::Char('D') => return QuitDecision::DiscardAndQuit,
                Key::Char('c') | Key::Char('C') | Key::Escape => {
                    self.status_msg = String::from("Operação cancelada.");
                    return QuitDecision::Cancel;
                }
                Key::None => continue,
                _ => {
                    self.status_msg = String::from("Operação cancelada.");
                    return QuitDecision::Cancel;
                }
            }
        }
    }

    pub fn new(filename: Option<String>) -> Self {
        let (cols, rows) = get_terminal_size();
        let screen_rows = if rows > 2 { rows - 2 } else { 1 };

        let mut editor = Self {
            rows: Vec::new(),
            cursor_x: 0,
            cursor_y: 0,
            row_offset: 0,
            col_offset: 0,
            screen_cols: cols,
            screen_rows,
            filename: filename.clone(),
            dirty: false,
            status_msg: String::from("★ Kvant (kv) v0.1.0 — ^S Gravar | ^X Sair | ^P/^N Navegar"),
            quit_confirmation: 0,
        };

        if let Some(ref path) = filename {
            if let Ok(content) = fs::read_to_string(path) {
                editor.rows = content.lines().map(|s| s.to_string()).collect();
                editor.status_msg =
                    format!("Arquivo carregado: {} ({} linhas)", path, editor.rows.len());
            }
        }

        if editor.rows.is_empty() {
            editor.rows.push(String::new());
        }

        editor
    }

    pub fn save(&mut self) -> io::Result<()> {
        if self.filename.is_none() {
            self.status_msg = String::from("Aviso: Nenhum nome de arquivo especificado.");
            return Ok(());
        }

        let path = self.filename.as_ref().unwrap();
        let mut content = self.rows.join("\n");
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }

        fs::write(path, content)?;
        self.dirty = false;
        self.status_msg = format!(
            "✓ Gravado com sucesso: {} ({} linhas)",
            path,
            self.rows.len()
        );
        Ok(())
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_y >= self.rows.len() {
            self.rows.push(String::new());
        }

        let row = &mut self.rows[self.cursor_y];
        let mut char_idx = 0;
        let mut byte_idx = 0;
        for ch in row.chars() {
            if char_idx == self.cursor_x {
                break;
            }
            char_idx += 1;
            byte_idx += ch.len_utf8();
        }

        row.insert(byte_idx, c);
        self.cursor_x += 1;
        self.dirty = true;
    }

    pub fn insert_newline(&mut self) {
        if self.cursor_y >= self.rows.len() {
            self.rows.push(String::new());
            self.cursor_y += 1;
            self.cursor_x = 0;
            self.dirty = true;
            return;
        }

        let row = &mut self.rows[self.cursor_y];
        let mut char_idx = 0;
        let mut byte_idx = 0;
        for ch in row.chars() {
            if char_idx == self.cursor_x {
                break;
            }
            char_idx += 1;
            byte_idx += ch.len_utf8();
        }

        let remainder = row.split_off(byte_idx);
        self.cursor_y += 1;
        self.rows.insert(self.cursor_y, remainder);
        self.cursor_x = 0;
        self.dirty = true;
    }

    pub fn delete_char_backward(&mut self) {
        if self.cursor_x == 0 && self.cursor_y == 0 {
            return;
        }

        if self.cursor_x > 0 {
            let row = &mut self.rows[self.cursor_y];
            let mut char_idx = 0;
            let mut byte_idx = 0;
            let mut prev_byte_idx = 0;
            for ch in row.chars() {
                if char_idx == self.cursor_x {
                    break;
                }
                prev_byte_idx = byte_idx;
                char_idx += 1;
                byte_idx += ch.len_utf8();
            }

            row.remove(prev_byte_idx);
            self.cursor_x -= 1;
            self.dirty = true;
        } else if self.cursor_y > 0 {
            // Merge with previous line
            let current_row = self.rows.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.rows[self.cursor_y].chars().count();
            self.rows[self.cursor_y].push_str(&current_row);
            self.dirty = true;
        }
    }

    pub fn delete_char_forward(&mut self) {
        if self.cursor_y >= self.rows.len() {
            return;
        }

        let row_len = self.rows[self.cursor_y].chars().count();
        if self.cursor_x < row_len {
            let row = &mut self.rows[self.cursor_y];
            let mut char_idx = 0;
            let mut byte_idx = 0;
            for ch in row.chars() {
                if char_idx == self.cursor_x {
                    break;
                }
                char_idx += 1;
                byte_idx += ch.len_utf8();
            }
            row.remove(byte_idx);
            self.dirty = true;
        } else if self.cursor_y + 1 < self.rows.len() {
            // Join next line into current
            let next_row = self.rows.remove(self.cursor_y + 1);
            self.rows[self.cursor_y].push_str(&next_row);
            self.dirty = true;
        }
    }

    pub fn kill_to_end_of_line(&mut self) {
        if self.cursor_y >= self.rows.len() {
            return;
        }
        let row = &mut self.rows[self.cursor_y];
        let mut char_idx = 0;
        let mut byte_idx = 0;
        for ch in row.chars() {
            if char_idx == self.cursor_x {
                break;
            }
            char_idx += 1;
            byte_idx += ch.len_utf8();
        }

        if byte_idx < row.len() {
            row.truncate(byte_idx);
            self.dirty = true;
        } else if self.cursor_y + 1 < self.rows.len() {
            // Already at end of line: kill newline by joining next line
            let next_row = self.rows.remove(self.cursor_y + 1);
            self.rows[self.cursor_y].push_str(&next_row);
            self.dirty = true;
        }
    }

    pub fn kill_entire_line(&mut self) {
        if self.rows.is_empty() {
            return;
        }
        if self.rows.len() == 1 {
            self.rows[0].clear();
            self.cursor_x = 0;
        } else {
            self.rows.remove(self.cursor_y);
            if self.cursor_y >= self.rows.len() {
                self.cursor_y = self.rows.len() - 1;
            }
            self.cursor_x = 0;
        }
        self.dirty = true;
    }

    pub fn clamp_cursor(&mut self) {
        if self.cursor_y >= self.rows.len() {
            self.cursor_y = if self.rows.is_empty() {
                0
            } else {
                self.rows.len() - 1
            };
        }
        let line_len = if self.rows.is_empty() {
            0
        } else {
            self.rows[self.cursor_y].chars().count()
        };
        if self.cursor_x > line_len {
            self.cursor_x = line_len;
        }

        // Adjust scrolling offsets
        if self.cursor_y < self.row_offset {
            self.row_offset = self.cursor_y;
        }
        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }

        if self.cursor_x < self.col_offset {
            self.col_offset = self.cursor_x;
        }
        if self.cursor_x >= self.col_offset + self.screen_cols {
            self.col_offset = self.cursor_x - self.screen_cols + 1;
        }
    }

    pub fn render(&mut self) -> io::Result<()> {
        let (cols, rows) = get_terminal_size();
        self.screen_cols = cols;
        self.screen_rows = if rows > 2 { rows - 2 } else { 1 };
        self.clamp_cursor();

        let mut output = String::with_capacity(cols * rows * 2);
        // Hide cursor, move home
        output.push_str("\x1b[?25l\x1b[H");

        // 1. Text rows
        for y in 0..self.screen_rows {
            let file_row = y + self.row_offset;
            if file_row < self.rows.len() {
                let row_str = &self.rows[file_row];
                let chars: Vec<char> = row_str.chars().collect();
                if chars.len() > self.col_offset {
                    let end_idx = (self.col_offset + self.screen_cols).min(chars.len());
                    let slice: String = chars[self.col_offset..end_idx].iter().collect();
                    output.push_str(&slice);
                }
            } else {
                output.push('~');
            }
            output.push_str("\x1b[K\r\n");
        }

        // 2. Status bar (reverse video)
        output.push_str("\x1b[7m");
        let name = self.filename.as_deref().unwrap_or("[Sem Nome]");
        let dirty_flag = if self.dirty { " [+]" } else { "" };
        let left_status = if !self.status_msg.is_empty() {
            format!(" {}", self.status_msg)
        } else {
            format!(
                " Kvant | {}{} - {} linha(s)",
                name,
                dirty_flag,
                self.rows.len()
            )
        };
        let right_status = format!("Ln {}, Col {} ", self.cursor_y + 1, self.cursor_x + 1);

        let pad_len = if self.screen_cols > left_status.len() + right_status.len() {
            self.screen_cols - left_status.len() - right_status.len()
        } else {
            0
        };

        output.push_str(&left_status);
        for _ in 0..pad_len {
            output.push(' ');
        }
        output.push_str(&right_status);
        output.push_str("\x1b[0m\r\n");

        // 3. Help footer (persistent shortcuts)
        output.push_str("\x1b[1;30;47m");
        let help = " ^S Salvar  ^X Sair  ^P/^N Linha  ^B/^F Cursor  ^A/^E Início/Fim  ^K Cortar ";
        output.push_str(help);
        if self.screen_cols > help.len() {
            for _ in 0..(self.screen_cols - help.len()) {
                output.push(' ');
            }
        }
        output.push_str("\x1b[0m");

        // 4. Place hardware cursor
        let cursor_screen_y = self.cursor_y - self.row_offset + 1;
        let cursor_screen_x = self.cursor_x - self.col_offset + 1;
        output.push_str(&format!(
            "\x1b[{};{}H\x1b[?25h",
            cursor_screen_y, cursor_screen_x
        ));

        let mut stdout = io::stdout();
        stdout.write_all(output.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Help and Usage Messages
// -----------------------------------------------------------------------------
pub fn show_help() {
    println!(
        r#"★ KVANT (kv) — Micro Terminal Text Editor ★
Padrão Gosplan / CCCP — 100% Rust Puro sem dependências externas

USO:
  kv [opções] [arquivo]
  kvant [opções] [arquivo]

OPÇÕES:
  -h, --help       Exibe esta mensagem de ajuda
  -v, --version    Exibe a versão do Kvant

NAVEGAÇÃO E ATALHOS DE TECLADO:
  Setas ou ^P/^N   Move o cursor para Linha Anterior (Cima) / Próxima (Baixo)
  Setas ou ^B/^F   Move o cursor para Caractere Anterior (Esq) / Próximo (Dir)
  Home ou ^A       Início da linha atual
  End ou ^E        Fim da linha atual
  PageUp / PageDn  Rola página acima / abaixo
  ^K               Corta/apaga do cursor até o final da linha
  ^U               Limpa a linha inteira
  ^D ou Delete     Apaga o caractere à frente do cursor
  Backspace ou ^H  Apaga o caractere atrás do cursor
  ^S               Salva o arquivo atual
  ^X ou ^Q         Sai do editor (avisa se houver alterações não salvas)

EXEMPLOS:
  kv novo_arquivo.md
  kv /tmp/anotacoes.txt
"#
    );
}

pub fn show_version() {
    println!("kvant 0.1.0-dev.1 (Apache-2.0)");
}

// -----------------------------------------------------------------------------
// Entry Point Run Function
// -----------------------------------------------------------------------------
pub fn run() {
    let args: Vec<String> = env::args().collect();
    let mut filename = None;

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" | "help" => {
                show_help();
                process::exit(0);
            }
            "-v" | "--version" | "version" => {
                show_version();
                process::exit(0);
            }
            path => {
                filename = Some(path.to_string());
            }
        }
    }

    let _guard = match RawModeGuard::enter() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Erro ao inicializar modo raw do terminal: {}", e);
            process::exit(1);
        }
    };

    let mut editor = Editor::new(filename);

    loop {
        if let Err(e) = editor.render() {
            eprintln!("Erro de renderização: {}", e);
            break;
        }

        match read_key() {
            Key::Ctrl('x') | Key::Ctrl('q') => match editor.confirm_quit_action(read_key) {
                QuitDecision::SaveAndQuit => {
                    let _ = editor.save();
                    break;
                }
                QuitDecision::DiscardAndQuit => {
                    break;
                }
                QuitDecision::Cancel => {
                    continue;
                }
            },
            Key::Ctrl('s') => {
                let _ = editor.save();
                editor.quit_confirmation = 0;
            }
            Key::Ctrl('k') => {
                editor.kill_to_end_of_line();
                editor.quit_confirmation = 0;
            }
            Key::Ctrl('u') => {
                editor.kill_entire_line();
                editor.quit_confirmation = 0;
            }
            Key::Ctrl('d') | Key::Delete => {
                editor.delete_char_forward();
                editor.quit_confirmation = 0;
            }
            Key::Backspace => {
                editor.delete_char_backward();
                editor.quit_confirmation = 0;
            }
            Key::Enter => {
                editor.insert_newline();
                editor.quit_confirmation = 0;
            }
            Key::Tab => {
                for _ in 0..4 {
                    editor.insert_char(' ');
                }
                editor.quit_confirmation = 0;
            }
            Key::Char(c) => {
                editor.insert_char(c);
                editor.quit_confirmation = 0;
            }
            Key::Up | Key::Ctrl('p') => {
                if editor.cursor_y > 0 {
                    editor.cursor_y -= 1;
                }
                editor.quit_confirmation = 0;
            }
            Key::Down | Key::Ctrl('n') => {
                if editor.cursor_y + 1 < editor.rows.len() {
                    editor.cursor_y += 1;
                }
                editor.quit_confirmation = 0;
            }
            Key::Left | Key::Ctrl('b') => {
                if editor.cursor_x > 0 {
                    editor.cursor_x -= 1;
                } else if editor.cursor_y > 0 {
                    editor.cursor_y -= 1;
                    editor.cursor_x = editor.rows[editor.cursor_y].chars().count();
                }
                editor.quit_confirmation = 0;
            }
            Key::Right | Key::Ctrl('f') => {
                let row_len = if editor.rows.is_empty() {
                    0
                } else {
                    editor.rows[editor.cursor_y].chars().count()
                };
                if editor.cursor_x < row_len {
                    editor.cursor_x += 1;
                } else if editor.cursor_y + 1 < editor.rows.len() {
                    editor.cursor_y += 1;
                    editor.cursor_x = 0;
                }
                editor.quit_confirmation = 0;
            }
            Key::Home | Key::Ctrl('a') => {
                editor.cursor_x = 0;
                editor.quit_confirmation = 0;
            }
            Key::End | Key::Ctrl('e') => {
                if !editor.rows.is_empty() {
                    editor.cursor_x = editor.rows[editor.cursor_y].chars().count();
                }
                editor.quit_confirmation = 0;
            }
            Key::PageUp => {
                editor.cursor_y = editor.cursor_y.saturating_sub(editor.screen_rows);
                editor.quit_confirmation = 0;
            }
            Key::PageDown => {
                editor.cursor_y =
                    (editor.cursor_y + editor.screen_rows).min(editor.rows.len().saturating_sub(1));
                editor.quit_confirmation = 0;
            }
            Key::None | Key::Escape | Key::Ctrl(_) => {
                // No action
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Unit Tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_insert_and_delete() {
        let mut editor = Editor::new(None);
        editor.insert_char('h');
        editor.insert_char('i');
        assert_eq!(editor.rows[0], "hi");
        assert_eq!(editor.cursor_x, 2);

        editor.delete_char_backward();
        assert_eq!(editor.rows[0], "h");
        assert_eq!(editor.cursor_x, 1);
    }

    #[test]
    fn test_editor_newline_and_merge() {
        let mut editor = Editor::new(None);
        for c in "line1".chars() {
            editor.insert_char(c);
        }
        editor.insert_newline();
        for c in "line2".chars() {
            editor.insert_char(c);
        }
        assert_eq!(editor.rows.len(), 2);
        assert_eq!(editor.rows[0], "line1");
        assert_eq!(editor.rows[1], "line2");

        editor.cursor_x = 0;
        editor.delete_char_backward();
        assert_eq!(editor.rows.len(), 1);
        assert_eq!(editor.rows[0], "line1line2");
    }

    #[test]
    fn test_editor_kill_to_end() {
        let mut editor = Editor::new(None);
        for c in "hello world".chars() {
            editor.insert_char(c);
        }
        editor.cursor_x = 5;
        editor.kill_to_end_of_line();
        assert_eq!(editor.rows[0], "hello");
    }

    #[test]
    fn test_editor_kill_entire_line() {
        let mut editor = Editor::new(None);
        for c in "first line".chars() {
            editor.insert_char(c);
        }
        editor.insert_newline();
        for c in "second line".chars() {
            editor.insert_char(c);
        }
        assert_eq!(editor.rows.len(), 2);
        editor.kill_entire_line();
        assert_eq!(editor.rows.len(), 1);
        assert_eq!(editor.rows[0], "first line");
    }

    #[test]
    fn test_confirm_quit_clean_editor() {
        let mut editor = Editor::new(None);
        editor.dirty = false;
        let decision = editor.confirm_quit_action(|| Key::None);
        assert_eq!(decision, QuitDecision::DiscardAndQuit);
    }

    #[test]
    fn test_confirm_quit_save() {
        let mut editor = Editor::new(None);
        editor.dirty = true;
        let decision_s = editor.confirm_quit_action(|| Key::Char('s'));
        assert_eq!(decision_s, QuitDecision::SaveAndQuit);

        let decision_upper_s = editor.confirm_quit_action(|| Key::Char('S'));
        assert_eq!(decision_upper_s, QuitDecision::SaveAndQuit);
    }

    #[test]
    fn test_confirm_quit_discard() {
        let mut editor = Editor::new(None);
        editor.dirty = true;
        let decision_d = editor.confirm_quit_action(|| Key::Char('d'));
        assert_eq!(decision_d, QuitDecision::DiscardAndQuit);

        let decision_upper_d = editor.confirm_quit_action(|| Key::Char('D'));
        assert_eq!(decision_upper_d, QuitDecision::DiscardAndQuit);
    }

    #[test]
    fn test_confirm_quit_cancel() {
        let mut editor = Editor::new(None);
        editor.dirty = true;

        let decision_c = editor.confirm_quit_action(|| Key::Char('c'));
        assert_eq!(decision_c, QuitDecision::Cancel);
        assert!(editor.status_msg.contains("cancelada"));

        let decision_esc = editor.confirm_quit_action(|| Key::Escape);
        assert_eq!(decision_esc, QuitDecision::Cancel);

        let decision_other = editor.confirm_quit_action(|| Key::Char('x'));
        assert_eq!(decision_other, QuitDecision::Cancel);
    }
}
