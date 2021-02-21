use derive_more::{Deref, From, Into};
use gdnative::prelude::ToVariant;
use std::sync::Arc;

#[derive(Into, From, Deref)]
pub struct Arcer<T: ToVariant>(Arc<T>);
