use std::hash::Hash;

pub mod api;
pub mod data;
pub mod external;
pub mod view;

const BASE_CONTENT_DIV_ID: &str = "base-content";

pub fn format_id_to_htmx_target_(id: String) -> String {
    format!("#{id}")
}

pub trait GetTableData {
    fn get_table_data(&self) -> Vec<String>;

    fn get_data_id(&self) -> String;

    fn get_headers() -> Vec<String>;
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
{
    pub fn new(records: Vec<T>) -> TableData<T> {
        let headers = T::get_headers();
        if records.is_empty() {
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
