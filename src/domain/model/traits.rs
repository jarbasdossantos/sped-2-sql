use async_trait::async_trait;
use diesel::result::Error;
use std::fmt::Debug;

#[async_trait]
pub trait Saveable: Debug + Send + Sync {
    async fn save(&self) -> std::result::Result<i32, Error>;
}

#[async_trait]
pub trait Findable: Debug + Send + Sync {
    async fn find(&self) -> std::result::Result<Option<Self>, Error>
    where
        Self: Sized;
}

pub trait Displayable: Debug + Send + Sync {
    fn display_fields(&self) -> Vec<(String, String)>;
}

pub trait Dataable: Debug + Send + Sync {
    fn data_fields(&self) -> Vec<&str>;
}