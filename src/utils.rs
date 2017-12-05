use std::collections::HashMap;
use std::hash::Hash;

pub fn hashmap_insertions<K, V>(
    new_hm: &HashMap<K, V>,
    old_hm: &HashMap<K, V>,
    include_modification: bool
) -> Vec<(K, V)>
    where
        K: Clone + Eq + Hash,
        V: PartialEq + Clone
{
    let mut insertions: Vec<(K, V)> = Vec::new();
    for (k, v) in new_hm.iter() {
        let diff = if let Some(ref old_v) = old_hm.get(k) {
            if include_modification && *old_v != v {
                true
            } else {
                false
            }
        } else {
            true
        };
        if diff {
            insertions.push((k.clone(), v.clone()));
        }
    }
    insertions
}
