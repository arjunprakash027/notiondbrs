use pyo3::prelude::*;
mod notion_utils;
mod utils;
mod notion_class;

#[pymodule]
fn notiondbrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<notion_class::NotionClient>()?;
    Ok(())
}
