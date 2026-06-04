use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};

pub(crate) fn get_hash<T: Hash>(hash: &T) -> u64 {
    let hasher = &mut DefaultHasher::new();
    hash.hash(hasher);
    hasher.finish()
}

pub(crate) fn extract_vals<K, V>(map: &mut BTreeMap<K, V>) -> impl Iterator<Item = V>
where
    K: Ord,
{
    map.extract_if(.., |_, _| true).map(|(_k, v)| v)
}

pub(crate) fn extract_flat_vals<K1, K2, V>(
    map: &mut BTreeMap<K1, BTreeMap<K2, V>>,
) -> impl Iterator<Item = V>
where
    K1: Ord,
    K2: Ord,
{
    map.iter_mut()
        .flat_map(|(_k, m)| m.extract_if(.., |_, _| true).map(|(_k, v)| v))
}
