#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Dice {
    pub core: DiceType,
    pub modifier: i64
}
impl Dice {
    pub fn new(core: DiceType) -> Dice {
        Dice::with_modifier(core, 0)
    }
    pub fn with_modifier(core: DiceType, modifier: i64) -> Dice {
        Dice {
            core, modifier
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D12
}