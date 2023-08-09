use std::collections::HashMap;
use anyhow::{anyhow, Result};
use crate::{
    adapters::hashmap_trait::HashmapExtensionTrait,
    frameworks::errors::multipart_error::MultipartError,
};

impl<V> HashmapExtensionTrait<V> for HashMap<String, V> {
    fn get_result(&mut self, key: &str) -> Result<V> {
        self.remove(key)
            .ok_or(anyhow!(MultipartError::DataNotFound))
    }
}