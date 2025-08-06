use std::collections::{
    HashMap,
    BTreeMap
};

use anyhow::Result;
use notion_client::{endpoints::{
    databases::query::{request::QueryDatabaseRequestBuilder, response::QueryDatabaseResponse}, pages::create::request::{self, CreateAPageRequest, CreateAPageRequestBuilder}, search::title::{
        request::{Filter, SearchByTitleRequestBuilder, Sort, SortDirection, Timestamp},
        response::PageOrDatabase,
    }, Client
}, objects::{page::PageProperty, parent::Parent}};

use notion_client::objects::{
    rich_text::{
        RichText,
        Text
    }
};


pub fn setup_notion_client(notion_token: &str) -> Result<Client> {
    let client = Client::new(notion_token.to_string(), None)?; //using the default reqwest client provided by notion_client crate
    //let db = client.databases.retreive_a_database(db_id).await?;
    println!("Connected to client");
    Ok(client)
}

pub async fn get_data_from_database(client: Client, db_id: &str) -> Result<QueryDatabaseResponse> {
    let request = QueryDatabaseRequestBuilder::default();
    
    let res = client
        .databases
        .query_a_database(db_id, request.build().unwrap())
        .await?;

    Ok(res)
}

pub async fn insert_data_to_notion(client: Client,upload_data: BTreeMap<String, Vec<String>>, db_id: String) -> Result<()> {
    
    let first_key = upload_data.keys().next().cloned();
    println!("{:#?} is the Key Column",first_key);
    for (key, values) in upload_data.iter() {
        let is_first = Some(key) == first_key.as_ref();
        convert_to_page_and_upload(&client ,key, values, &db_id, is_first).await?;
    }
    Ok(())
}

pub async fn convert_to_page_and_upload(client: &Client ,key: &String, values: &Vec<String>, db_id: &String, is_first: bool) -> Result<()> {

    for value in values.iter(){
        let mut properties = BTreeMap::new();
        
        if is_first {
            properties.insert(
                key.to_string(),
                PageProperty::Title{ id: None,
                    title: vec![RichText::Text { 
                        text: Text { content: value.to_string(), link: None }, annotations: None, plain_text: None, href: None
                    }],
                },
            );
        } else {
            properties.insert(
                key.to_string(),
                PageProperty::RichText{ id: None,
                    rich_text: vec![RichText::Text { 
                        text: Text { content: value.to_string(), link: None }, annotations: None, plain_text: None, href: None
                    }],
                },
            );
        }
        
        let request = CreateAPageRequest {
            parent: Parent::DatabaseId { database_id: db_id.to_string(), },
            icon: None,
            cover: None,
            children: None,
            properties: properties
        };
        
        if let Err(e) = client.pages.create_a_page(request).await {
            // Print the entire error object
            eprintln!("Upload failed: {e}");
            return Err(e.into());
        }
}
Ok(())
}

pub async fn get_all_databases(client: Client) -> Result<Vec<(String, String)>> {
    let mut request = SearchByTitleRequestBuilder::default();
    request.filter(Filter {
        value: notion_client::endpoints::search::title::request::FilterValue::Database,
        property: notion_client::endpoints::search::title::request::FilterProperty::Object,
    });
    request.sort(Sort {
        timestamp: Timestamp::LastEditedTime,
        direction: SortDirection::Ascending,
    });

    let response = client
        .search
        .search_by_title(request.build().unwrap())
        .await?;

    let databases = response
        .results
        .iter()
        .filter_map(|entry| {
            if let PageOrDatabase::Database(db) = entry {
                let id = db.id.clone().unwrap_or_default();
                let name = db
                    .title
                    .get(0)
                    .and_then(|text_block| text_block.plain_text().clone())
                    .unwrap_or_else(|| "<Untitled>".to_string());
                Some((id, name))
            } else {
                None
            }
        })
        .collect();

    Ok(databases)
}
