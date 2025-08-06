from notion_utils import NotionClient
from dotenv import load_dotenv
import os

load_dotenv()

NOTION_TOKEN = os.environ.get("NOTION_TOKEN") or "none"
DB_ID = os.environ.get("DB_ID") or "none"
PAGE_ID = os.environ.get("PAGE_ID") or "none"

client = NotionClient(NOTION_TOKEN)
databases = client.get_all_databases()
print(databases)

data = client.get_data_from_database(DB_ID)

print(type(data))

upload_data = {
    "Name": ['1','2','3'],
    "b": ['4','5','6'],
}
client.merge_data(upload_data, DB_ID)
#client.insert_data(upload_data, DB_ID)
#client.insert_data(upload_data, PAGE_ID, new_db=True)