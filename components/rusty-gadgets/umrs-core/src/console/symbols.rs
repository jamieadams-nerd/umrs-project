/// Unified Symbols and Glyphs for the UMRS Design System.
///
/// This module centralizes Unicode characters to ensure visual
/// consistency across CLI output, logs, and UI headers.
///
// use crate::console::symbols::{SLIM, BOLD, icons};
// fn draw_tree() {
//     println!("{} My Folder", icons::FOLDER);
//     println!("{}{}{} Root Mount", SLIM.tl, SLIM.h, icons::MOUNT);
// }
pub struct BoxStyle {
    pub tl: char,     // Top Left
    pub tr: char,     // Top Right
    pub bl: char,     // Bottom Left
    pub br: char,     // Bottom Right
    pub h: char,      // Horizontal
    pub v: char,      // Vertical
    pub conn_l: char, // Connector Left (├)
    pub conn_t: char, // Connector Top (┬)
    pub conn_m: char, // Connector Middle/Cross (┼)
    pub conn_b: char, // Connector Bottom (┴)
    pub conn_r: char, // Connector Right (┤)
}

/// Standard "Slim" box set for general UI borders.
pub const SLIM: BoxStyle = BoxStyle {
    tl: '┌',
    tr: '┐',
    bl: '└',
    br: '┘',
    h: '─',
    v: '│',
    conn_l: '├',
    conn_t: '┬',
    conn_m: '┼',
    conn_b: '┴',
    conn_r: '┤',
};

/// "Bold" box set for high-priority alerts or headers.
pub const BOLD: BoxStyle = BoxStyle {
    tl: '┏',
    tr: '┓',
    bl: '┗',
    br: '┛',
    h: '━',
    v: '┃',
    conn_l: '┣',
    conn_t: '┳',
    conn_m: '╋',
    conn_b: '┻',
    conn_r: '┫',
};

/// "Rounded" box set - corners are rounded
pub const ROUNDED: BoxStyle = BoxStyle {
    tl: '\u{2560}',
    tr: '\u{256E}',
    bl: '\u{2570}',
    br: '\u{256F}',
    h: '━',
    v: '┃',
    conn_l: '┣',
    conn_t: '┳',
    conn_m: '╋',
    conn_b: '┻',
    conn_r: '┫',
};

/// Global Technical Icons
pub mod icons {
    pub const ACTOR: &str = "\u{1FBC5}"; // Symbol: 🯅  Name: STICK FIGURE
    pub const CHECK: &str = "✔";
    pub const CROSS: &str = "✘";
    pub const FOLDER_OPEN: &str = "📂";
    pub const FOLDER: &str = "📁";
    pub const INFO: &str = "ℹ";
    pub const MOUNT_SECURE: &str = "🔐"; // Useful for LUKS mounts
    pub const MOUNT: &str = "⏏";
    pub const WARNING: &str = "⚠";
}
