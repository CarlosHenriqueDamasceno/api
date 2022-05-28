use crate::dto::{CustomerInputDTO, CustomerOutputDTO};
use crate::repositories::CustomerRepository;
use crate::entities::customer::Customer;
use std::error::Error;

pub struct GetCustomer<T> where T: CustomerRepository {
    repository: T
}

impl<T> GetCustomer<T> where T: CustomerRepository{
    
    pub fn execute(&self, id:u32) -> Result<Customer, Box<dyn Error>>{
        let result = self.repository.get(id)?;
        Ok(result)
    }

    pub fn new (repository:T) -> GetCustomer<T> {
        GetCustomer { repository }
    }
}

pub struct SaveCustomer<'a, T> where T: CustomerRepository {
    repository: &'a mut T
}

impl<'a, T> SaveCustomer<'a, T> where T: CustomerRepository{
    
    pub fn execute(&mut self, data:CustomerInputDTO) -> Result<CustomerOutputDTO, Box<dyn Error>>{
        let result = self.repository.save(data)?;
        Ok(result)
    }

    pub fn new (repository:&'a mut T) -> SaveCustomer<T> {
        SaveCustomer { repository }
    }
}
