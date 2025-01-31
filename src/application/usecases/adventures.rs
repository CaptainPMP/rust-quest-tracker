use std::sync::Arc;
use anyhow::Result;
use crate::domain::{repositories::adventures::AdventurersRepository, value_object::adventurer_model::RegisterAdventurerModel};

pub struct AdventurersUserCase<T>
where 
    T: AdventurersRepository + Send + Sync
{
    pub adventurers_repository: Arc<T>,
}   

impl <T> AdventurersUserCase<T>
where 
    T: AdventurersRepository + Send + Sync
{
    pub fn new(adventurers_repository: Arc<T>) -> Self {
        Self {
            adventurers_repository,
        }
    }

    pub async fn register(&self, mut register_adventurer_model: RegisterAdventurerModel) -> Result<i32> {
        unimplemented!()
    }
}