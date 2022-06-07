use crate::dtos::{CustomerInputDTO, CustomerOutputDTO};
use crate::repositories::CustomerRepository;
use std::error::Error;


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
