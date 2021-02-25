use crate::map::fow::{Uncover, FOW};
use crate::observable::Observable;
use crate::observable::Observer;
use gdnative::prelude::*;
use std::sync::Arc;
use strum_macros::AsRefStr;

#[derive(Copy, Clone, PartialEq, Eq, AsRefStr)]
pub enum FOWSignal {
    Uncover,
}

impl From<&Uncover> for FOWSignal {
    fn from(_: &Uncover) -> Self {
        FOWSignal::Uncover
    }
}

pub struct FOWObserver {
    owner: Ref<Node, Shared>,
}

impl Observer<Uncover> for FOWObserver {
    fn notify(&self, uncover: &Uncover) {
        unsafe {
            self.owner.assume_safe().emit_signal(
                FOWSignal::from(uncover),
                &[uncover.coordinates().to_variant()],
            );
        }
    }
}

impl FOWObserver {
    pub fn new(fow: &FOW, owner: Ref<Node, Shared>) -> Arc<Self> {
        let observer = Arc::new(FOWObserver { owner });
        fow.observers().register(&observer);
        observer
    }
}
