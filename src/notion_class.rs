use crate::notion_utils::*;
use crate::utils::*;
use notion_client::endpoints::Client as NativeNotionClient;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use tokio::runtime::Runtime;

#[pyclass]
pub struct NotionClient {
    client: NativeNotionClient,
}

#[pymethods]
impl NotionClient {
    #[new]
    pub fn new(notion_token: String) -> PyResult<Self> {
        let client = setup_notion_client(&notion_token)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(NotionClient { client })
    }

    pub fn get_all_databases(&self) -> PyResult<Vec<(String, String)>> {
        let rt =
            Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let databases = rt
            .block_on(get_all_databases(self.client.clone()))
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(databases)
    }

    pub fn get_data(&self, db_id: &str) -> PyResult<HashMap<String, Vec<String>>> {
        let rt =
            Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let data = rt
            .block_on(get_data_from_database(self.client.clone(), db_id))
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let data_hashmap = convert_notion_result_to_hashmap(&data)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(data_hashmap)
    }

    pub fn merge_data(&self, upload_data: &Bound<'_, PyDict>) -> PyResult<()> {
        let input_hashmap = convert_pydict_to_hashmap(upload_data);
        println!("Python hashmap repr: {:?}", input_hashmap);
        Ok(())
    }
}
