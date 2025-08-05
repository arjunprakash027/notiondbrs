use polars::prelude::*;
use std::collections::HashMap;
use anyhow::Result;
use notion_client::endpoints::{
    databases::query::{
        response::{
            QueryDatabaseResponse,
        },
    }
};
use notion_client::objects::{
    page::PageProperty
};

pub fn convert_notion_result_to_hashmap(
    result: &QueryDatabaseResponse
) -> Result<HashMap<String, Vec<String>>> {
    
    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    
    for page in result.results.iter() {
        for (prop_name, prop_val) in &page.properties {
            let val = match prop_val {
                PageProperty::Title{id:_,title:title_arr} => {
                    title_arr
                        .iter()
                        .filter_map(|t| t.plain_text().clone())
                        .collect::<Vec<String>>()
                }
                PageProperty::RichText {id:_, rich_text: rich_text_arr} => {
                    rich_text_arr
                        .iter()
                        .filter_map(|rt| rt.plain_text().clone())
                        .collect::<Vec<String>>()
                }
                PageProperty::Number {id:_, number: number_arr} => {
                    vec![number_arr.as_ref().map_or(" ".to_string(), |n| n.to_string())]
                }
                _ => vec![" ".to_string()]
            };
            
            data.entry(prop_name.clone()).or_insert_with(Vec::new).extend(val);
        }
    }
    
    Ok(data)
}

pub fn hashmap_to_polars(data: &HashMap<String, Vec<String>>) -> Result<DataFrame> {
    let cols_vec : Vec<Column> = data
        .into_iter()
        .map(|(col_name, vals)| {
            let s = Series::new(col_name.into(), vals);
            Column::from(s)
        })
        .collect();
    
    let df = DataFrame::new(cols_vec)?;
    Ok(df) 
}
