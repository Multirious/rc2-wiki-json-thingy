use crate::prelude::*;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Default)]
pub struct PathMap<'a>(pub BTreeMap<VecDeque<&'a str>, Json>);

impl<'a> PathMap<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_json(input: &'a Json) -> Self {
        fn from_json_inner<'a>(
            result_map: &mut PathMap<'a>,
            current_path: &'a Json,
            current_path_str: VecDeque<&'a str>,
        ) {
            let input_as_obj = current_path.as_object().unwrap();
            for (k, v) in input_as_obj {
                if v.is_object() {
                    let mut deeper_path_str = current_path_str.clone();
                    deeper_path_str.push_back(k);
                    from_json_inner(result_map, v, deeper_path_str);
                    continue;
                }
                let mut deeper_path_str = current_path_str.clone();
                deeper_path_str.push_back(k);
                result_map.0.insert(deeper_path_str, v.clone());
            }
        }

        let mut result_map = PathMap::default();
        from_json_inner(&mut result_map, input, vec![].into());
        result_map
    }

    pub fn to_json(&self) -> Json {
        let mut json = Json::Object(serde_json::Map::new());
        for (path, v) in self.0.iter() {
            let mut entry = &mut json;
            for path_seg in path {
                entry = &mut entry[path_seg];
            }
            *entry = v.clone();
        }
        json
    }
}
