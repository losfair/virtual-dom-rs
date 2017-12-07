use std::collections::BTreeMap;

pub fn btreemap_insertions<'a, K, V>(
    new_map: &'a BTreeMap<K, V>,
    old_map: &'a BTreeMap<K, V>,
    include_modification: bool
) -> Vec<(&'a K, &'a V)>
    where K: PartialEq + Ord, V: PartialEq
{
    let mut insertions = Vec::new();

    for (k, new_value) in new_map.iter() {
        let need_insert = if let Some(ref old_value) = old_map.get(k) {
            if !include_modification || *old_value == new_value {
                false
            } else {
                true
            }
        } else {
            true
        };
        if need_insert {
            insertions.push((k, new_value));
        }
    }

    insertions
}
