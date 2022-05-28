use std::error::Error;

use crate::{entities::{car::Car, customer::Customer}, dto::{CustomerInputDTO, CustomerOutputDTO}};
pub trait CarRepository {
    fn get(&self, id:u32) -> Result<Car, &'static str>;
}

pub trait CustomerRepository {
    fn get(&self, id:u32) -> Result<Customer, &'static str>;
    fn save(&mut self, data:CustomerInputDTO) -> Result<CustomerOutputDTO, Box<dyn Error>>;
}