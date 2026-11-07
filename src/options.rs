//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Info,
    Abbrev,
    Aranges,
    Frames,
    Loc,
}

#[derive(Debug, Clone)]
pub struct DwarfdumpOptions {
    pub mode: Mode,
    pub debug_info: Option<String>,
}

impl Default for DwarfdumpOptions {
    fn default() -> Self { Self { mode: Mode::Default, debug_info: None } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(DwarfdumpOptions::default().mode, Mode::Default); }
}
