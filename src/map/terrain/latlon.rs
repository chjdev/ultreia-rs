use crate::saturating_from::SaturatingInto;
use std::cmp::Ordering;

pub trait LatLon {
    fn normalized(&self) -> f64;
    fn abs(&self) -> Self;
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Default)]
pub struct Latitude {
    normalized: f64,
    value: f64,
}

impl Latitude {
    fn new(normalized: f64) -> Self {
        Self {
            value: (normalized * std::f64::consts::FRAC_PI_2).sin() * 90.,
            normalized,
        }
    }
}

impl Into<f64> for Latitude {
    fn into(self) -> f64 {
        self.value
    }
}

impl LatLon for Latitude {
    fn normalized(&self) -> f64 {
        self.normalized
    }
    fn abs(&self) -> Self {
        Latitude::new(self.normalized().abs())
    }
}

impl PartialEq<f64> for Latitude {
    fn eq(&self, other: &f64) -> bool {
        Into::<f64>::into(*self).eq(other)
    }
}

impl PartialOrd<f64> for Latitude {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        Into::<f64>::into(*self).partial_cmp(other)
    }
}

impl SaturatingInto<Latitude> for f64 {
    fn saturating_from(ny: &f64) -> Latitude {
        Latitude::new(ny.clamp(-1., 1.))
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Default)]
pub struct Longitude {
    normalized: f64,
    value: f64,
}

impl Longitude {
    fn new(normalized: f64) -> Self {
        Self {
            value: normalized * 180.,
            normalized,
        }
    }
}

impl Into<f64> for Longitude {
    fn into(self) -> f64 {
        self.value
    }
}

impl LatLon for Longitude {
    fn normalized(&self) -> f64 {
        self.normalized
    }

    fn abs(&self) -> Self {
        Self::new(self.normalized().abs())
    }
}

impl PartialEq<f64> for Longitude {
    fn eq(&self, other: &f64) -> bool {
        Into::<f64>::into(*self).eq(other)
    }
}

impl PartialOrd<f64> for Longitude {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        Into::<f64>::into(*self).partial_cmp(other)
    }
}

impl SaturatingInto<Longitude> for f64 {
    fn saturating_from(nx: &f64) -> Longitude {
        Longitude::new(nx.clamp(-1., 1.))
    }
}
