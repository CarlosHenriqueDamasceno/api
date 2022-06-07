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
