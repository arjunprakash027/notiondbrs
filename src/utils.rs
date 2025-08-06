use anyhow::Result;
use notion_client::endpoints::databases::query::response::QueryDatabaseResponse;
use notion_client::objects::page::PageProperty;
use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

pub fn convert_notion_result_to_hashmap(
    result: &QueryDatabaseResponse,
) -> Result<HashMap<String, Vec<String>>> {
    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    for page in result.results.iter() {
        for (prop_name, prop_val) in &page.properties {
            let val = match prop_val {
                PageProperty::Title {
                    id: _,
                    title: title_arr,
                } => title_arr
                    .iter()
                    .filter_map(|t| t.plain_text().clone())
                    .collect::<Vec<String>>(),
                PageProperty::RichText {
                    id: _,
                    rich_text: rich_text_arr,
                } => rich_text_arr
                    .iter()
                    .filter_map(|rt| rt.plain_text().clone())
                    .collect::<Vec<String>>(),
                PageProperty::Number {
                    id: _,
                    number: number_arr,
                } => {
                    vec![
                        number_arr
                            .as_ref()
                            .map_or(" ".to_string(), |n| n.to_string()),
                    ]
                }
                _ => vec![" ".to_string()],
            };

            data.entry(prop_name.clone())
                .or_insert_with(Vec::new)
                .extend(val);
        }
    }

    Ok(data)
}

pub fn convert_pydict_to_hashmap(
    pydata: &Bound<'_, PyDict>,
) -> Result<HashMap<String, Vec<String>>> {
    let mut hashmap_data: HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in pydata.iter() {
        let key_str = key.extract::<String>()?;

        let py_list = value
            .downcast::<PyList>()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let rust_vec: Vec<String> = py_list
            .iter()
            .map(|item| item.extract::<String>())
            .collect::<PyResult<_>>()?;

        hashmap_data.insert(key_str, rust_vec);
    }

    Ok(hashmap_data)
}
