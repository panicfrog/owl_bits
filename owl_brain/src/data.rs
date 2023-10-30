
use std::collections::HashSet;
use std::sync::Arc;
use dashmap::DashMap;

pub type DataRef = Arc<Data>;

pub enum Data {
    Str(String),
    Double(f64),
    Float(f32),
    Bool(bool),
    Date(i64),
    DataRef(DataRef),
    Array(Vec<DataRef>),
    Object(DashMap<String, DataRef>),
}

impl Data {
    pub fn mark(&self, marked: &mut HashSet<*const Data>) {
        if marked.insert(self as *const Data) {
            match self {
                Data::DataRef(data_ref) => data_ref.mark(marked),
                Data::Array(array) => {
                    for data_ref in array {
                        data_ref.mark(marked);
                    }
                }
                Data::Object(map) => {
                    for data_ref in map.iter() {
                        data_ref.mark(marked);
                    }
                }
                _ => {}
            }
        }
    }
}

