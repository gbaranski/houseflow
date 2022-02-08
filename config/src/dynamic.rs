use arc_swap::ArcSwap;
use arc_swap::Guard;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Config<T: crate::Config>(Arc<ArcSwap<T>>);

impl<T: crate::Config> Config<T> {
    pub fn new(config: T) -> Self {
        Self(Arc::new(ArcSwap::from(Arc::new(config))))
    }

    pub fn get(&self) -> Guard<Arc<T>> {
        self.0.load()
    }

    pub fn update(&self, config: T) {
        self.0.store(Arc::new(config));
    }
}
