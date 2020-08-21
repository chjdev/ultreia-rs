use crate::coordinate::indexed::Indexed;

pub struct Road;

#[derive(Default)]
pub struct RoadNetwork {
    pub map: Indexed<Road>
}
