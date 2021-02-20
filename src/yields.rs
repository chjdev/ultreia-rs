use crate::saturating_from::SaturatingInto;
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Yield(u8);

const PERCENT100_YIELD: f64 = (u8::max_value() / 2) as f64;
const PERCENT200_YIELD: f64 = u8::max_value() as f64;

impl Yield {
    pub fn percent(&self) -> f64 {
        (self.0 as f64) / PERCENT100_YIELD
    }
}

impl Into<f64> for Yield {
    fn into(self) -> f64 {
        self.percent()
    }
}

impl PartialEq<f64> for Yield {
    fn eq(&self, other: &f64) -> bool {
        Into::<f64>::into(*self).eq(other)
    }
}

impl PartialOrd<f64> for Yield {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        Into::<f64>::into(*self).partial_cmp(other)
    }
}

impl SaturatingInto<Yield> for f64 {
    fn saturating_from(value: &f64) -> Yield {
        Yield((value.clamp(0., 2.) * PERCENT100_YIELD) as u8)
    }
}
