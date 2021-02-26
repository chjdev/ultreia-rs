use crate::godot::emit_deferred::EmitDeferred;
use crate::map::buildings::{BuildingCreated, BuildingDestroyed, Buildings};
use crate::observable::{Observable, Observer};
use gdnative::prelude::*;
use std::sync::Arc;
use strum_macros::AsRefStr;

#[derive(Copy, Clone, PartialEq, Eq, AsRefStr)]
pub enum BuildingsSignal {
    Created,
    Destroyed,
}

impl From<&BuildingCreated> for BuildingsSignal {
    fn from(_: &BuildingCreated) -> Self {
        BuildingsSignal::Created
    }
}

impl From<&BuildingDestroyed> for BuildingsSignal {
    fn from(_: &BuildingDestroyed) -> Self {
        BuildingsSignal::Destroyed
    }
}

pub struct BuildingsObserver {
    owner: Ref<Node, Shared>,
}

impl Observer<BuildingCreated> for BuildingsObserver {
    fn notify(&self, event: &BuildingCreated) {
        self.owner.emit_deferred(
            BuildingsSignal::from(event),
            &[event.coordinate.to_variant(), event.tile_name.to_variant()],
        )
    }
}

impl Observer<BuildingDestroyed> for BuildingsObserver {
    fn notify(&self, event: &BuildingDestroyed) {
        self.owner.emit_deferred(
            BuildingsSignal::from(event),
            &[event.coordinate.to_variant()],
        );
    }
}

impl BuildingsObserver {
    pub fn new(buildings: &Buildings, owner: Ref<Node, Shared>) -> Arc<Self> {
        let observer = Arc::new(Self { owner });
        Observable::<BuildingCreated>::observers(buildings).register(&observer);
        Observable::<BuildingDestroyed>::observers(buildings).register(&observer);
        observer
    }
}
