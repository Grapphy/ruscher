use crate::error::Error;
use async_trait::async_trait;

pub mod google;
pub use google::GoogleSearch;

#[async_trait]
pub trait LeecherModule {
    async fn urls(&self) -> Result<Vec<String>, Error>;
}
