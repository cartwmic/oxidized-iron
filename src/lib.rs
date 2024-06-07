use std::hash::Hash;

pub mod api;
pub mod data;
pub mod external;
pub mod view;

pub trait GetTableData {
    fn get_table_data(&self) -> Vec<String>;

    fn get_data_id(&self) -> String;
}

pub trait GetUrlPrefix {
    fn get_url_prefix(&self) -> String;
}

#[derive(Clone)]
pub struct TableData<T>
where
    T: Clone,
    T: Hash,
    T: Eq,
    T: GetTableData,
    T: GetUrlPrefix,
{
    headers: Vec<String>,
    records: Vec<T>,
}

impl<T> TableData<T>
where
    T: Clone,
    T: Hash,
    T: Eq,
    T: GetTableData,
    T: GetUrlPrefix,
{
    pub fn new(headers: Vec<String>, records: Vec<T>) -> TableData<T> {
        if headers.is_empty() || records.is_empty() {
            panic!()
        }
        if headers.len() != records.first().unwrap().get_table_data().len() {
            panic!();
        } else {
            TableData { headers, records }
        }
    }

    pub fn get_headers(&self) -> &Vec<String> {
        self.headers.as_ref()
    }

    pub fn get_records(&self) -> &Vec<T> {
        self.records.as_ref()
    }
}
