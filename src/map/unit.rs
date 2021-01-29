use crate::coordinate::indexed::Indexed;

pub struct Road;

pub struct RoadNetwork {
    pub map: Indexed<Road>,
}
