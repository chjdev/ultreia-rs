use crate::coordinate::indexed::Indexed;
use crate::tile::TileInstance;

pub type TileMap = Indexed<Box<dyn TileInstance>>;
