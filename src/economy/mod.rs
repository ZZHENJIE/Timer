pub mod finviz;

#[derive(Debug)]
pub struct DataItem {
    pub date_timestamp: i64,
    pub event: String,
    pub importance: u8,
    pub forecast: Option<String>,
    pub actual: Option<String>,
    pub previous: Option<String>,
}
