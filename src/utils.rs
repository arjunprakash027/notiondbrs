use polars::prelude::*;

pub fn create_df() -> PolarsResult<DataFrame> {
    let s = Series::new("companies".into(), &["google","microsoft","apple"]);
    let c = Series::new("market_cap".into(), &[1,2,3]);
    
    let df = DataFrame::new(vec![s.into(),c.into()])?;    
    
    Ok(df)
}