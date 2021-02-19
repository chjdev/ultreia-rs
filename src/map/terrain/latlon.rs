use std::convert::TryFrom;

#[derive(PartialEq, PartialOrd, Copy, Clone, Default)]
pub struct Latitude(f64);

impl Latitude {
    fn new(latitude: f64) -> Self {
        Latitude(latitude)
    }

    pub fn saturating_from(latitude: f64) -> Self {
        Latitude::new(latitude.clamp(-90., 90.))
    }
}

impl TryFrom<f64> for Latitude {
    type Error = &'static str;

    fn try_from(ny: f64) -> Result<Self, Self::Error> {
        let latitude = (ny * std::f64::consts::FRAC_PI_2).sin() * 90.;
        if latitude < -90. || latitude > 90. {
            Err("value outside of latitude range")
        } else {
            Ok(Latitude::new(latitude))
        }
    }
}

impl Into<f64> for &Latitude {
    fn into(self) -> f64 {
        self.0
    }
}

impl Into<f64> for Latitude {
    fn into(self) -> f64 {
        (&self).into()
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Default)]
pub struct Longitude(f64);

impl Longitude {
    fn new(longitude: f64) -> Self {
        Longitude(longitude)
    }

    pub fn saturating_from(longitude: f64) -> Self {
        Longitude::new(longitude.clamp(-180., 180.))
    }
}

impl TryFrom<f64> for Longitude {
    type Error = &'static str;

    fn try_from(nx: f64) -> Result<Self, Self::Error> {
        let longitude: f64 = nx * 180.;
        if longitude < -180. || longitude > 180. {
            Err("value outside of latitude range")
        } else {
            Ok(Longitude::new(longitude))
        }
    }
}

impl Into<f64> for &Longitude {
    fn into(self) -> f64 {
        self.0
    }
}

impl Into<f64> for Longitude {
    fn into(self) -> f64 {
        (&self).into()
    }
}
