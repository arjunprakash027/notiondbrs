import notiondbrs

NOTION_TOKEN = 
DB_ID = 

client = notiondbrs.NotionClient(NOTION_TOKEN)
print(client)

db = client.get_all_databases()

print(db)
