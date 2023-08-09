use anyhow::Result;

pub trait HashmapExtensionTrait<K> {
    fn get_result(&mut self, key: &str) -> Result<K>;
}