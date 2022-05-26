use crate::entities::{Car, Customer};
pub trait CarRepository {
    fn get(&self, id:u32) -> Result<&Car, &'static str>;
}

pub trait CustomerRepository {
    fn get(&self, id:u32) -> Result<&Customer, &'static str>;
}