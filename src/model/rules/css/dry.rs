use crate::model::rules::LineResult;

use super::Rule;

pub struct Dry;

impl Rule for Dry {
    fn get_name(&self) -> &str {
        "CSS-DRY"
    }
    fn get_description(&self) -> &str {
        // TODO add example of dry css, ressources and
        "try to refactor your css so that multiple dont have the same attributes and rather abstract them to save bandwidth, for example you might have four arrow direction and they all have the same style except for the arrow direction, consider an arrow abstraction and resuse everything for each arrow except the direction."
    }
    fn apply(&self, input: &str) -> Option<std::vec::Vec<LineResult>> {
        // TODO implement dry css rule here
        todo!()
    }
}
