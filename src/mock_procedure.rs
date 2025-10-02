pub struct MockProcedure<T, AM> {
    taxonomies: Vec<T>,
    asset_models: Vec<AM>,
}

impl <T,AM> MockProcedure<T,AM> {
    pub fn taxonomies(&self) -> &[T] {
        &self.taxonomies
    }
    pub fn asset_models(&self) -> &[AM] {
        &self.asset_models
    }
}