use gdnative::prelude::{Dictionary, OwnedToVariant, ToVariant, ToVariantEq};
use std::collections::HashMap;

pub fn make_dict<K, V>(from: &HashMap<K, V>) -> Dictionary
where
    K: OwnedToVariant + ToVariantEq + ToVariant,
    V: OwnedToVariant + ToVariant,
{
    let mut dict = Dictionary::new();
    dict.extend(from.iter());
    dict.into_shared()
}
