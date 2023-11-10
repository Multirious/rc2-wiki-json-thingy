#![cfg(test)]

use std::collections::HashSet;

use crate::{path_map::PathMap, prelude::*};
use serde_json::json;

#[test]
fn tranpose() {
    let input = json!({
        "1": {
            "key1": "value1_1",
            "e_table": {
                "key2": "value2_1",
                "e_table1": {
                    "key3": "value3_1",
                    "key4": "value4_1"
                }
            }
        },
        "98": {
            "key1": "value1_98",
            "e_table": {
                "key2": "value2_98",
                "e_table1": {
                    "key3": "value3_98",
                    "key4": "value4_98"
                }
            }
        }
    });
    let input_path_map = PathMap::from_json(&input);
    let special = HashSet::from_iter(["e_table", "e_table1"].iter().map(|s| s.to_string()));
    let tranposed_path_map = super::tranpose(&input_path_map, &special);
    let tranposed = tranposed_path_map.to_json();
    let expected_output = json!({
        "key1": {
            "1": "value1_1",
            "98": "value1_98"
        },
        "e_table": {
            "key2": {
                "1": "value2_1",
                "98": "value2_98"
            },
            "e_table1": {
                "key3": {
                    "1": "value3_1",
                    "98": "value3_98"
                },
                "key4": {
                    "1": "value4_1",
                    "98": "value4_98"
                }
            }
        }
    });
    assert_eq!(tranposed, expected_output);
}

#[test]
fn tranpose2() {
    let input = json!({
        "1": {
            "key1": "value1_1",
            "e_table": {
                "key2": "value2_1",
                "e_table1": {
                    "key3": "value3_1",
                    "key4": "value4_1"
                },
                "e_table2": {
                    "key5": "value5_1",
                    "key6": "value6_1"
                },
                "table1": {
                    "key7": "value7_1",
                    "key8": "value8_1"
                }
            }
        },
        "98": {
            "key1": "value1_98",
            "e_table": {
                "key2": "value2_98",
                "e_table1": {
                    "key3": "value3_98",
                    "key4": "value4_98"
                },
                "e_table2": {
                    "key5": "value5_98",
                    "key6": "value6_98"
                },
                "table1": {
                    "key7": "value7_98",
                    "key8": "value8_98"
                }
            }
        }
    });
    let input_path_map = PathMap::from_json(&input);
    let to_not_expanded = HashSet::from_iter(
        ["e_table", "e_table1", "e_table2"]
            .iter()
            .map(|s| s.to_string()),
    );
    let tranposed_path_map = super::tranpose(&input_path_map, &to_not_expanded);
    let tranposed = tranposed_path_map.to_json();
    println!("{}", serde_json::to_string_pretty(&tranposed).unwrap());
    let expected_output = json!(
    {
        "key1": {
            "1": "value1_1",
            "98": "value1_98"
        },
        "e_table": {
            "key2": {
                "1": "value2_1",
                "98": "value2_98"
            },
            "e_table1": {
                "key3": {
                    "1": "value3_1",
                    "98": "value3_98"
                },
                "key4": {
                    "1": "value4_1",
                    "98": "value4_98"
                }
            },
            "e_table2": {
                "key5": {
                    "1": "value5_1",
                    "98": "value5_98"
                },
                "key6": {
                    "1": "value6_1",
                    "98": "value6_98"
                }
            },
            "table1": {
                "1": {
                    "key7": "value7_1",
                    "key8": "value8_1"
                },
                "98": {
                    "key7": "value7_98",
                    "key8": "value8_98"
                }
            }
        }
    });
    assert_eq!(tranposed, expected_output);
}
