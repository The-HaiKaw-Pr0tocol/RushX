//!
//! ## `rushx_term::ansi` <ins>module</ins>: ANSI / VT100 Escape Sequence Parser
//!
//! Stateful ANSI / VT100 escape sequence parser and companion text-style
//! types for the RushX terminal emulator.  Processes raw PTY byte streams
//! and emits structured [`Action`] values describing styled text output and
//! discrete control events (bell, backspace, carriage return, screen clear).
//!
//! ### Recognized sequences
//!
//! | Sequence                    | Meaning                                      |
//! |-----------------------------|----------------------------------------------|
//! | `\x1b[…m` (SGR)             | Foreground / background colors, decoration   |
//! | `\x1b[2J`, `\x1b[3J` (ED)  | Erase display (clear screen)                 |
//! | `\x1b]…BEL` / `\x1b]…ST`   | OSC — consumed and discarded                 |
//! | `\x07` BEL                  | Terminal bell                                |
//! | `\x08` BS                   | Backspace — erase last character             |
//! | `\r` (standalone)           | Carriage return — rewind to line start       |
//! | All other CSI sequences     | Silently discarded                           |
//!
//! ## Metadata
//!
//! - **File**: src/rushx_term/ansi.rs
//! - **Module**: rushx_term::ansi
//! - **Last Update**: 02/26/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol
//!

/*=============================================================================*/

/* ── 16-Color Palette ─────────────────────────────────────────────────────── */

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     ANSI_PALETTE: [(f64, f64, f64); 16]
/// ```
/// Catppuccin Mocha 16-color terminal palette, normalized to `[0.0, 1.0]`
/// (R, G, B).
///
/// Indices 0-7 are the standard ANSI colors; 8-15 are their bright variants.
/// Values are derived from the Catppuccin Mocha specification to complement
/// the terminal theme defined in [`super::config`].
///
/// | Index | Name          | Hex       |
/// |-------|---------------|-----------|
/// | 0     | Black         | `#1e1e2e` |
/// | 1     | Red           | `#f38ba8` |
/// | 2     | Green         | `#a6e3a1` |
/// | 3     | Yellow        | `#f9e2af` |
/// | 4     | Blue          | `#89b4fa` |
/// | 5     | Magenta/Pink  | `#f5c2e7` |
/// | 6     | Cyan          | `#94e2d5` |
/// | 7     | White         | `#cdd6f4` |
/// | 8     | Bright Black  | `#585b70` |
/// | 9-14  | Bright 1-6    | same hues |
/// | 15    | Bright White  | `#b4befe` |
///
pub const ANSI_PALETTE: [(f64, f64, f64); 16] = [
    (0.118, 0.118, 0.180), //-- 0  Black         #1e1e2e (Base)        --//
    (0.953, 0.545, 0.659), //-- 1  Red            #f38ba8               --//
    (0.651, 0.890, 0.631), //-- 2  Green          #a6e3a1               --//
    (0.976, 0.886, 0.686), //-- 3  Yellow         #f9e2af               --//
    (0.537, 0.706, 0.980), //-- 4  Blue           #89b4fa               --//
    (0.961, 0.761, 0.906), //-- 5  Magenta / Pink #f5c2e7               --//
    (0.580, 0.886, 0.835), //-- 6  Cyan           #94e2d5               --//
    (0.804, 0.839, 0.957), //-- 7  White          #cdd6f4 (Text)        --//
    (0.345, 0.357, 0.439), //-- 8  Bright Black   #585b70 (Surface2)    --//
    (0.953, 0.545, 0.659), //-- 9  Bright Red     #f38ba8               --//
    (0.651, 0.890, 0.631), //-- 10 Bright Green   #a6e3a1               --//
    (0.976, 0.886, 0.686), //-- 11 Bright Yellow  #f9e2af               --//
    (0.537, 0.706, 0.980), //-- 12 Bright Blue    #89b4fa               --//
    (0.961, 0.761, 0.906), //-- 13 Bright Magenta #f5c2e7               --//
    (0.580, 0.886, 0.835), //-- 14 Bright Cyan    #94e2d5               --//
    (0.706, 0.745, 0.996), //-- 15 Bright White   #b4befe (Lavender)    --//
];

/* ══ Color ════════════════════════════════════════════════════════════════════ */

///
/// #### **<ins>Enum</ins>**
/// ```Rust
///     Color { Default, Named(u8), Rgb(u8, u8, u8), Indexed(u8) }
/// ```
/// A color value produced by an SGR sequence.
///
/// Covers all three encoding schemes used by modern terminal emulators:
/// the classic 16-color set, the xterm 256-color palette, and 24-bit RGB.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    /// Terminal's configured default
    Default,

    /// One of the 16 standard ANSI colors.
    ///
    /// - `0-7`: `\x1b[30m`-`\x1b[37m` (standard)
    /// - `8-15`: `\x1b[90m`-`\x1b[97m` (bright variants)
    Named(u8),

    /// 24-bit true color (`\x1b[38;2;r;g;bm` / `\x1b[48;2;r;g;bm`).
    Rgb(u8, u8, u8),

    /// 256-color palette index (`\x1b[38;5;nm` / `\x1b[48;5;nm`).
    Indexed(u8),
}

impl Color {
    ///
    /// #### **<ins>Method</ins>**
    /// ```Rust
    ///     fn to_rgb_f64(&self, default: (f64, f64, f64)) -> (f64, f64, f64)
    /// ```
    /// Converts a `Color` to a normalized `(R, G, B)` tuple in `[0.0, 1.0]`,
    /// suitable for passing directly to `cairo::Context::set_source_rgb`.
    ///
    /// ### Arguments
    /// - `default`: Fallback color used when `self` is `Color::Default`
    ///
    /// ### Index Mapping
    /// - `Named(0..=15)` → [`ANSI_PALETTE`]
    /// - `Indexed(0..=15)` → [`ANSI_PALETTE`] (identical to `Named`)
    /// - `Indexed(16..=231)` → 6×6×6 color cube:
    ///   `idx = n - 16`, `r = idx/36`, `g = (idx%36)/6`, `b = idx%6`;
    ///   each channel: `0 → 0.0`, `n → (55 + 40·n) / 255`
    /// - `Indexed(232..=255)` → grayscale ramp:
    ///   `value = (8 + 10·(n - 232)) / 255`
    ///
    pub fn to_rgb_f64(&self, default: (f64, f64, f64)) -> (f64, f64, f64) {
        match *self {
            Color::Default => default,

            Color::Named(n) => ANSI_PALETTE.get(n as usize).copied().unwrap_or(default),

            Color::Rgb(r, g, b) => (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0),

            Color::Indexed(n) => {
                let n = n as usize;
                if n < 16 {
                    //-- Same as Named --//
                    ANSI_PALETTE.get(n).copied().unwrap_or(default)
                } else if n < 232 {
                    //-- 6×6×6 color cube --//
                    let idx = n - 16;
                    let ri = idx / 36;
                    let gi = (idx % 36) / 6;
                    let bi = idx % 6;
                    let ch = |v: usize| -> f64 {
                        if v == 0 {
                            0.0
                        } else {
                            (55 + 40 * v) as f64 / 255.0
                        }
                    };
                    (ch(ri), ch(gi), ch(bi))
                } else {
                    //-- Grayscale ramp (232-255) --//
                    let v = (8.0 + 10.0 * (n - 232) as f64) / 255.0;
                    (v, v, v)
                }
            }
        }
    }
}

/* ══ SgrState ════════════════════════════════════════════════════════════════ */

///
/// #### **<ins>Struct</ins>**
/// ```Rust
///     SgrState { fg, bg, bold, dim, italic, underline, blink, reverse, strikethrough }
/// ```
/// Active SGR (Select Graphic Rendition) text attributes.
///
/// Tracks the cumulative text style after processing all `\x1b[…m` sequences
/// seen so far.  Resets to [`SgrState::default`] on `\x1b[0m` (or bare
/// `\x1b[m`).
///
#[derive(Debug, Clone, PartialEq)]
pub struct SgrState {
    /// Foreground (text) color.
    pub fg: Color,
    /// Background (cell fill) color.
    pub bg: Color,
    /// Bold intensity (`\x1b[1m`).
    pub bold: bool,
    /// Dim / reduced intensity (`\x1b[2m`).
    pub dim: bool,
    /// Italic (`\x1b[3m`).
    pub italic: bool,
    /// Underline (`\x1b[4m`).
    pub underline: bool,
    /// Slow / rapid blink (`\x1b[5m` / `\x1b[6m`).
    pub blink: bool,
    /// Reverse video — swaps fg and bg (`\x1b[7m`).
    pub reverse: bool,
    /// Strikethrough (`\x1b[9m`).
    pub strikethrough: bool,
}

impl Default for SgrState {
    fn default() -> Self {
        SgrState {
            fg: Color::Default,
            bg: Color::Default,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            reverse: false,
            strikethrough: false,
        }
    }
}

impl SgrState {
    ///
    /// #### **<ins>Method</ins>**
    /// ```Rust
    ///     fn apply_params(&mut self, params: &str)
    /// ```
    /// Parses a semicolon-delimited SGR parameter string and mutates `self`
    /// in place.
    ///
    /// ### Arguments
    /// - `params`: Raw parameter substring from a `\x1b[…m` sequence —
    ///   e.g. `"0"`, `"1;32"`, `"38;5;160"`, `"38;2;255;128;0"`.
    ///   An empty string is treated as `"0"` (full reset).
    ///
    /// ### Supported Parameters
    ///
    /// | Value(s)      | Effect                            |
    /// |---------------|-----------------------------------|
    /// | `0` / `` (empty) | Full reset                     |
    /// | `1`           | Bold                              |
    /// | `2`           | Dim                               |
    /// | `3`           | Italic                            |
    /// | `4`           | Underline                         |
    /// | `5`, `6`      | Blink                             |
    /// | `7`           | Reverse video                     |
    /// | `9`           | Strikethrough                     |
    /// | `22`          | Normal intensity (clears bold/dim)|
    /// | `23`-`29`     | Clear italic/underline/blink/etc. |
    /// | `30`-`37`     | Standard fg color                 |
    /// | `38;5;n`      | 256-color fg                      |
    /// | `38;2;r;g;b`  | 24-bit RGB fg                     |
    /// | `39`          | Default fg                        |
    /// | `40`-`47`     | Standard bg color                 |
    /// | `48;5;n`      | 256-color bg                      |
    /// | `48;2;r;g;b`  | 24-bit RGB bg                     |
    /// | `49`          | Default bg                        |
    /// | `90`-`97`     | Bright fg (8-15)                  |
    /// | `100`-`107`   | Bright bg (8-15)                  |
    ///
    fn apply_params(&mut self, params: &str) {
        let raw: Vec<u32> = if params.is_empty() {
            vec![0]
        } else {
            params
                .split(';')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect()
        };

        let mut i = 0;
        while i < raw.len() {
            match raw[i] {
                0 => {
                    *self = SgrState::default();
                }
                1 => {
                    self.bold = true;
                }
                2 => {
                    self.dim = true;
                }
                3 => {
                    self.italic = true;
                }
                4 => {
                    self.underline = true;
                }
                5 | 6 => {
                    self.blink = true;
                }
                7 => {
                    self.reverse = true;
                }
                9 => {
                    self.strikethrough = true;
                }

                22 => {
                    self.bold = false;
                    self.dim = false;
                }
                23 => {
                    self.italic = false;
                }
                24 => {
                    self.underline = false;
                }
                25 => {
                    self.blink = false;
                }
                27 => {
                    self.reverse = false;
                }
                29 => {
                    self.strikethrough = false;
                }

                30..=37 => {
                    self.fg = Color::Named(raw[i] as u8 - 30);
                }

                38 if i + 1 < raw.len() => match raw[i + 1] {
                    5 if i + 2 < raw.len() => {
                        self.fg = Color::Indexed(raw[i + 2] as u8);
                        i += 2;
                    }
                    2 if i + 4 < raw.len() => {
                        self.fg = Color::Rgb(raw[i + 2] as u8, raw[i + 3] as u8, raw[i + 4] as u8);
                        i += 4;
                    }
                    _ => {}
                },

                39 => {
                    self.fg = Color::Default;
                }

                40..=47 => {
                    self.bg = Color::Named(raw[i] as u8 - 40);
                }

                48 if i + 1 < raw.len() => match raw[i + 1] {
                    5 if i + 2 < raw.len() => {
                        self.bg = Color::Indexed(raw[i + 2] as u8);
                        i += 2;
                    }
                    2 if i + 4 < raw.len() => {
                        self.bg = Color::Rgb(raw[i + 2] as u8, raw[i + 3] as u8, raw[i + 4] as u8);
                        i += 4;
                    }
                    _ => {}
                },

                49 => {
                    self.bg = Color::Default;
                }

                90..=97 => {
                    self.fg = Color::Named(raw[i] as u8 - 90 + 8);
                }

                100..=107 => {
                    self.bg = Color::Named(raw[i] as u8 - 100 + 8);
                }

                _ => {}
            }
            i += 1;
        }
    }
}

/* ══ StyledSpan ══════════════════════════════════════════════════════════════ */

///
/// #### **<ins>Struct</ins>**
/// ```Rust
///     StyledSpan { text: String, style: SgrState }
/// ```
/// A contiguous run of text characters sharing the same SGR attributes.
///
/// ### Fields
/// - `text`: The character content of this run.  May contain `\n` newlines.
/// - `style`: The active [`SgrState`] at the time these characters were emitted.
///
#[derive(Debug, Clone)]
pub struct StyledSpan {
    /// Text content (may include `\n`).
    pub text: String,
    /// SGR attributes active when this text was produced.
    pub style: SgrState,
}

/* ══ Action ══════════════════════════════════════════════════════════════════ */

///
/// #### **<ins>Enum</ins>**
/// ```Rust
///     Action { Text(StyledSpan), Bell, Backspace, CarriageReturn, EraseDisplay }
/// ```
/// A discrete action emitted by [`AnsiParser::feed`].
///
/// Callers apply each action to the terminal's screen buffer in order.
/// See [`apply_action`] for the canonical application logic.
///
pub enum Action {
    /// A run of styled text: append to the screen buffer.
    Text(StyledSpan),
    /// Terminal bell (`\x07`): caller should ring the system bell.
    Bell,
    /// Backspace (`\x08`): erase the last displayed character on the current line.
    Backspace,
    /// Carriage return (`\r` not followed by `\n`): rewind to the start of
    /// the current line without advancing to the next.
    CarriageReturn,
    /// Erase display (`\x1b[2J` / `\x1b[3J`): clear the entire screen buffer.
    EraseDisplay,
}

/* ══ Internal Escape State ═══════════════════════════════════════════════════ */

///
/// Internal parser state for escape sequence accumulation.
///
#[derive(Debug, Clone, PartialEq)]
enum EscapeState {
    /// Normal text input.
    Normal,
    /// `\x1b` seen: waiting for the sequence introducer character.
    Esc,
    /// Inside a CSI (`\x1b[`) sequence: accumulating parameter bytes.
    Csi(String),
    /// Inside an OSC (`\x1b]`) sequence: consuming until BEL or ST.
    Osc,
}

/* ══ AnsiParser ══════════════════════════════════════════════════════════════ */

///
/// #### **<ins>Struct</ins>**
/// ```Rust
///     AnsiParser { ... }
/// ```
/// Stateful ANSI / VT100 escape sequence parser.
///
/// Processes raw byte slices from a PTY master fd one chunk at a time,
/// maintaining current SGR attributes and escape-sequence parsing state
/// across successive calls.  Emits a sequence of [`Action`] values
/// describing how the terminal display should be updated.
///
pub struct AnsiParser {
    /// Current active text rendering attributes.
    sgr: SgrState,
    /// Current escape-sequence parsing state.
    escape_state: EscapeState,
}

impl AnsiParser {
    ///
    /// #### **<ins>Method</ins>**
    /// ```Rust
    ///     fn new() -> AnsiParser
    /// ```
    /// Creates a new `AnsiParser` with default SGR state and no active
    /// escape sequence in progress.
    ///
    pub fn new() -> Self {
        AnsiParser {
            sgr: SgrState::default(),
            escape_state: EscapeState::Normal,
        }
    }

    ///
    /// #### **<ins>Method</ins>**
    /// ```Rust
    ///     fn feed(&mut self, data: &[u8]) -> Vec<Action>
    /// ```
    /// Feeds a raw PTY byte slice into the parser and returns the resulting
    /// sequence of [`Action`] values.
    ///
    /// ### Arguments
    /// - `data`: Raw bytes read from the PTY master fd (one read-chunk)
    ///
    /// ### Returns
    /// A `Vec<Action>` with styled text spans and control events, in order.
    /// Consecutive printable characters sharing the same SGR state are merged
    /// into a single `Action::Text` span to minimize allocations.
    ///
    /// ### State Persistence
    /// Parser state (current SGR attributes and mid-sequence accumulation)
    /// persists across calls, so a sequence split across two PTY read chunks
    /// is handled correctly.
    ///
    pub fn feed(&mut self, data: &[u8]) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        let text = String::from_utf8_lossy(data);
        let mut chars = text.chars().peekable();

        let mut current_text = String::new();

        macro_rules! flush_text {
            () => {
                if !current_text.is_empty() {
                    actions.push(Action::Text(StyledSpan {
                        text: std::mem::take(&mut current_text),
                        style: self.sgr.clone(),
                    }));
                }
            };
        }

        while let Some(ch) = chars.next() {
            //-- Take ownership of the current state to avoid borrow conflicts --//
            let state = std::mem::replace(&mut self.escape_state, EscapeState::Normal);

            match state {
                /*----------------------------------------------------------------*/
                /* Normal text input                                               */
                /*----------------------------------------------------------------*/
                EscapeState::Normal => {
                    match ch {
                        '\x1b' => {
                            flush_text!();
                            self.escape_state = EscapeState::Esc;
                        }
                        '\x08' => {
                            flush_text!();
                            actions.push(Action::Backspace);
                            self.escape_state = EscapeState::Normal;
                        }
                        '\r' => {
                            flush_text!();
                            if chars.peek() == Some(&'\n') {
                                chars.next();
                                current_text.push('\n');
                            } else {
                                actions.push(Action::CarriageReturn);
                            }
                            self.escape_state = EscapeState::Normal;
                        }
                        '\x07' => {
                            flush_text!();
                            actions.push(Action::Bell);
                            self.escape_state = EscapeState::Normal;
                        }
                        '\x00' => {
                            //-- NUL: discard --//
                            self.escape_state = EscapeState::Normal;
                        }
                        _ => {
                            current_text.push(ch);
                            self.escape_state = EscapeState::Normal;
                        }
                    }
                }

                /*----------------------------------------------------------------*/
                /* ESC received — identify sequence introducer                    */
                /*----------------------------------------------------------------*/
                EscapeState::Esc => {
                    match ch {
                        '[' => {
                            self.escape_state = EscapeState::Csi(String::new());
                        }
                        ']' => {
                            self.escape_state = EscapeState::Osc;
                        }
                        _ => {
                            //-- Single-character ESC sequence: ignore and return to Normal --//
                            self.escape_state = EscapeState::Normal;
                        }
                    }
                }

                /*----------------------------------------------------------------*/
                /* CSI sequence: \x1b[ <params> <final-byte>                      */
                /* Parameter bytes: 0x30-0x3F (?  - @-1)                         */
                /* Final byte:      0x40-0x7E (@  - ~)                            */
                /*----------------------------------------------------------------*/
                EscapeState::Csi(mut params) => {
                    if ('@'..='~').contains(&ch) {
                        self.escape_state = EscapeState::Normal;

                        match ch {
                            'm' => {
                                self.sgr.apply_params(&params);
                            }
                            'J' if params == "2" || params == "3" => {
                                flush_text!();
                                actions.push(Action::EraseDisplay);
                            }
                            _ => {}
                        }
                    } else {
                        params.push(ch);
                        self.escape_state = EscapeState::Csi(params);
                    }
                }

                /*----------------------------------------------------------------*/
                /* OSC sequence: \x1b] … terminated by BEL or ST (\x1b\\)        */
                /* Used for: window title, hyperlinks, colour palette changes, …  */
                /* None of these affect text rendering, consume and discard.     */
                /*----------------------------------------------------------------*/
                EscapeState::Osc => {
                    match ch {
                        '\x07' => {
                            self.escape_state = EscapeState::Normal;
                        }
                        '\x1b' => {
                            if chars.peek() == Some(&'\\') {
                                chars.next();
                            }
                            self.escape_state = EscapeState::Normal;
                        }
                        _ => {
                            self.escape_state = EscapeState::Osc;
                        }
                    }
                }
            }
        }

        flush_text!();
        actions
    }
}

/* ══ Screen Buffer Helpers ═══════════════════════════════════════════════════ */

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     fn apply_action(buffer: &mut Vec<StyledSpan>, action: Action)
/// ```
/// Applies a single [`Action`] to an in-memory span buffer in place.
///
/// ### Arguments
/// - `buffer`: The terminal's accumulated screen buffer (a flat list of styled spans)
/// - `action`: The action to apply
///
/// ### Behavior
/// - `Text`: Merges into the last span if styles match (avoids fragmentation),
///   otherwise appends a new span.
/// - `Backspace`: Pops the last Unicode scalar from the last span without
///   crossing a newline boundary.
/// - `CarriageReturn`: Truncates the buffer back to the end of the last
///   `\n` newline (i.e. discards the current unsaved line).
/// - `EraseDisplay`: Clears the entire buffer.
/// - `Bell`: No-op, bell is surfaced as a `bool` return value from
///   `process_pty_output` for the caller to handle.
///
pub fn apply_action(buffer: &mut Vec<StyledSpan>, action: Action) {
    match action {
        Action::Text(span) => {
            if let Some(last) = buffer.last_mut() {
                if last.style == span.style {
                    last.text.push_str(&span.text);
                    return;
                }
            }
            buffer.push(span);
        }

        Action::Backspace => {
            while let Some(last) = buffer.last_mut() {
                if last.text.is_empty() {
                    buffer.pop();
                    continue;
                }
                if last.text.ends_with('\n') {
                    break;
                }
                last.text.pop();
                if last.text.is_empty() {
                    buffer.pop();
                }
                break;
            }
        }

        Action::CarriageReturn => {
            while let Some(last) = buffer.last_mut() {
                if let Some(nl_pos) = last.text.rfind('\n') {
                    last.text.truncate(nl_pos + 1);
                    return;
                }
                buffer.pop();
            }
        }

        Action::EraseDisplay => {
            buffer.clear();
        }

        Action::Bell => {}
    }
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     fn build_render_lines(spans: &[StyledSpan])
///         -> Vec<Vec<(String, SgrState)>>
/// ```
/// Splits a flat span buffer into a line-oriented structure for rendering.
///
/// ### Arguments
/// - `spans`: The terminal's accumulated screen buffer
///
/// ### Returns
/// A `Vec` where each element is a screen line, itself a `Vec` of
/// `(text_segment, style)` pairs, one entry per styled run within
/// that line.  The last element always represents the current (incomplete)
/// cursor line, which may be an empty vec.
///
/// ### Behavior
/// Newline characters embedded in span text are used as line delimiters.
/// A span that crosses a `\n` is split across consecutive lines; each
/// fragment inherits the original span's style.
///
pub fn build_render_lines(spans: &[StyledSpan]) -> Vec<Vec<(String, SgrState)>> {
    let mut lines: Vec<Vec<(String, SgrState)>> = vec![vec![]];

    for span in spans {
        let mut parts = span.text.split('\n').peekable();
        while let Some(part) = parts.next() {
            if !part.is_empty() {
                lines
                    .last_mut()
                    .unwrap()
                    .push((part.to_string(), span.style.clone()));
            }

            if parts.peek().is_some() {
                lines.push(vec![]);
            }
        }
    }

    lines
}
