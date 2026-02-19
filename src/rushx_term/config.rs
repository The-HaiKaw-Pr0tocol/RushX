//!
//! ## `rushx_term::config` <ins>module</ins>: Terminal Configuration
//!
//! Terminal emulator configuration constants. Centralizes window geometry,
//! color scheme, application identity, font settings, and shell invocation
//! paths. All tunable parameters live here to avoid magic numbers scattered
//! across UI and PTY code.
//!
//! ## Metadata
//!
//! - **File**: src/rushx_term/config.rs
//! - **Module**: rushx_term::config
//! - **Last Update**: 02/18/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol
//!

/*=============================================================================*/

/* ── Application Identity ─────────────────────────────────────────────────── */

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     APP_ID: &str = "haikaw.rushx.terminal"
/// ```
/// GTK Application ID for D-Bus registration.
///
pub const APP_ID: &str = "haikaw.rushx.terminal";

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     WINDOW_TITLE: &str = "RushX"
/// ```
/// Window title displayed in the title bar / window manager.
///
pub const WINDOW_TITLE: &str = "RushX";

/* ── Window Geometry ──────────────────────────────────────────────────────── */

///º
/// #### **<ins>Constant</ins>**
/// ```Rust
///     WINDOW_WIDTH: i32 = 800
/// ```
/// Default window width in pixels.
///
pub const WINDOW_WIDTH: i32 = 800;

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     WINDOW_HEIGHT: i32 = 500
/// ```
/// Default window height in pixels.
///
pub const WINDOW_HEIGHT: i32 = 500;

/* ── Color Scheme ─────────────────────────────────────────────────────────── */

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     BG_COLOR: (f64, f64, f64) = (0.117, 0.117, 0.180)
/// ```
/// Background color in (R, G, B)
///
/// ### Palette
/// Dark blue-gray `#1e1e2e`
///
pub const BG_COLOR: (f64, f64, f64) = (0.117, 0.117, 0.180);

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     FG_COLOR: (f64, f64, f64) = (0.804, 0.839, 0.957)
/// ```
/// Foreground (text) color in (R, G, B)
///
/// ### Palette
/// Light gray `#cdd6f4`
///
pub const FG_COLOR: (f64, f64, f64) = (0.804, 0.839, 0.957);

/* ── Font & Text Layout ───────────────────────────────────────────────────── */

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     FONT_FAMILY: &str = "monospace"
/// ```
/// Font family used for terminal text rendering (Cairo toy text API).
///
pub const FONT_FAMILY: &str = "monospace";

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     FONT_SIZE: f64 = 14.0
/// ```
/// Font size in Cairo units (roughly equivalent to points at 96 DPI).
///
pub const FONT_SIZE: f64 = 14.0;

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     LINE_HEIGHT: f64 = 18.0
/// ```
/// Vertical distance between baselines, in pixels.
///
pub const LINE_HEIGHT: f64 = 18.0;

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     TEXT_PADDING: f64 = 4.0
/// ```
/// Horizontal and vertical padding from the window edge, in pixels.
///
pub const TEXT_PADDING: f64 = 4.0;

/* ── Shell Invocation ─────────────────────────────────────────────────────── */

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     SHELL_PATH: &str = "/proc/self/exe"
/// ```
/// Path to the shell binary used inside the PTY child process.
///
/// ### Behavior
/// The terminal emulator re-invokes itself with `SHELL_FLAG` to launch
/// the RushX shell inside the PTY child. `/proc/self/exe` resolves to the
/// current binary on Linux.
///
pub const SHELL_PATH: &str = "/proc/self/exe";

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     SHELL_FLAG: &str = "--rushx-shell"
/// ```
/// CLI flag passed to the re-invoked binary to enter shell mode.
///
pub const SHELL_FLAG: &str = "--rushx-shell";

/* ── PTY I/O ──────────────────────────────────────────────────────────────── */

///
/// #### **<ins>Constant</ins>**
/// ```Rust
///     PTY_READ_BUF_SIZE: usize = 4096
/// ```
/// Read buffer size for the PTY master reader thread, in bytes.
///
pub const PTY_READ_BUF_SIZE: usize = 4096;
