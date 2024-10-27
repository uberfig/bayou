use std::collections::HashMap;

pub trait Headers {
    fn get(&self, key: &str) -> Option<String>;
}

pub struct ReqwestHeaders {
    pub headermap: reqwest::header::HeaderMap,
}

impl Headers for ReqwestHeaders {
    fn get(&self, key: &str) -> Option<String> {
        let val = self.headermap.get(key).map(|x| x.to_str())?;
        match val {
            Ok(x) => Some(x.to_string()),
            Err(_) => None,
        }
    }
}

pub struct HashMapHeaders {
    pub headermap: HashMap<String, String>,
}

impl Headers for HashMapHeaders {
    fn get(&self, key: &str) -> Option<String> {
        let val = self.headermap.get(key)?;
        Some(val.to_string())
    }
}
