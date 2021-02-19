use crate::saturating_from::SaturatingInto;
use derive_more::Into;
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Copy, Clone, Default, Into)]
pub struct Latitude(f64);

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
        let latitude = (ny * std::f64::consts::FRAC_PI_2).sin() * 90.;
        Latitude(latitude.clamp(-90., 90.))
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Default, Into)]
pub struct Longitude(f64);

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
        let longitude: f64 = nx * 180.;
        Longitude(longitude.clamp(-180., 180.))
    }
}
