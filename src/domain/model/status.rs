#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusCondition {
    Poison,
    Paralysis,
    Burn,
    Freeze,
    Sleep,
}
