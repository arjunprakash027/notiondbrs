use notion_client::endpoints::Client as NativeNotionClient;
use pyo3::prelude::*;
use tokio::runtime::Runtime;
use crate::notion_utils::*;

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
}