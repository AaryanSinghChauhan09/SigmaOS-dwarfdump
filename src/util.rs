//! Dwarfdump model — display DWARF debug information.

#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub compilation_units: usize,
    pub functions: usize,
    pub variables: usize,
    pub types: usize,
}

#[derive(Debug, Clone)]
pub struct Dwarfdump {
    pub show_aranges: bool,
    pub show_info: bool,
}

impl Default for Dwarfdump {
    fn default() -> Self { Self { show_aranges: false, show_info: true } }
}

impl Dwarfdump {
    pub fn new() -> Self { Self::default() }

    pub fn parse(&self, _data: &[u8]) -> DebugInfo {
        DebugInfo { compilation_units: 0, functions: 0, variables: 0, types: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let d = Dwarfdump::new();
        assert_eq!(d.parse(&[]).compilation_units, 0);
    }
}
