use ::dice::Dice;
use ::dice::DiceType;

pub struct Skill {
    name: String,
    dice: Option<Dice>
}
impl Skill {
    fn name(&self) -> &String {
        &self.name
    }
    fn dice(&self) -> Dice {
        self.dice.unwrap_or(Dice::with_modifier(DiceType::D4, -2))
    }
}