mod utils;
mod notion_utils;
pub const NOTION_TOKEN: &str = "your_notion_token_here";

#[tokio::main]
async fn main() {
    println!("I am blazzingly fast now!");
    let df = utils::create_df().expect("Failed to create DataFrame");
    println!("{:?}", df);
    let notion_client = notion_utils::setup_notion_client(NOTION_TOKEN)
        .await
        .expect("Failed to setup Notion client");
    
    let all_databases =  notion_utils::get_all_databases(notion_client).await.expect("Failed to get all databases");
    
    print!("{:#?}", all_databases);
    //println!("All databases: {:?}", all_db);
}