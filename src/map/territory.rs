use std::rc::Rc;

use crate::coordinate::indexed::Indexed;

pub struct Territory;

#[derive(Default)]
pub struct Territories {
    pub map: Indexed<Rc<Territory>>,
}
