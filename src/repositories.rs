use std::error::Error;

use crate::{entities::{car::Car, customer::Customer}, dtos::{CustomerInputDTO, CustomerOutputDTO}};
pub trait CarRepository {
    fn get(&mut self, id:u32) -> Result<Car, &'static str>;
    fn getAll(&mut self) -> Result<Vec<Car>, Box<dyn Error>>;
}

pub trait CustomerRepository {
    fn get(&self, id:u32) -> Result<Customer, &'static str>;
    fn save(&mut self, data:CustomerInputDTO) -> Result<CustomerOutputDTO, Box<dyn Error>>;
}