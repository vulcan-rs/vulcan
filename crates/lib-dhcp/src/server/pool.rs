use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoolParseError {}

#[derive(Debug)]
pub struct Pool {
    range: PoolRange,
    name: String,
}

impl TryFrom<(String, String)> for Pool {
    type Error = PoolParseError;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum PoolRangeParseError {}

#[derive(Debug)]
pub struct PoolRange {}

impl TryFrom<String> for PoolRange {
    type Error = PoolRangeParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}
