use gdnative::prelude::*;

use super::variant::good_variant::GOOD_ENUM;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Good;

impl Good {
    fn new(_owner: &Node) -> Self {
        Good {}
    }
}

#[methods]
impl Good {
    #[export]
    fn good_enum(&self, _owner: &Node) -> Dictionary<Unique> {
        GOOD_ENUM.duplicate()
    }
}
