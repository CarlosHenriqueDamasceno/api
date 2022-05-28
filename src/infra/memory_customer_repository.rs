use std::{collections::HashMap, error::Error};

use crate::{repositories::CustomerRepository, entities::customer::Customer, dtos::{CustomerOutputDTO, CustomerInputDTO}};

pub struct MemoryCustomerRepository{
    customers: HashMap<u32, Customer>
}

impl MemoryCustomerRepository {
    pub fn new() -> MemoryCustomerRepository{

        let mut customers = HashMap::new();

        customers.insert(1, Customer {
            id: 1,
            name: String::from("Carlos Henrique"),
            document: String::from("02547895456")
        });

        MemoryCustomerRepository { customers }
    }
}

impl CustomerRepository for MemoryCustomerRepository {
    fn get(&self, id:u32) -> Result<Customer, &'static str> {

        match self.customers.get(&id){
            Some(v) => {

                return Ok(Customer{
                    id:       v.id,
                    name:     v.name.to_owned(),
                    document: v.document.to_owned(),
                })
            },
            None => {
                return Err("There is no customer with this id!!");
            }
        };
    }

    fn save(&mut self, data:CustomerInputDTO) -> Result<CustomerOutputDTO, Box<dyn Error>> {
        
        let customer = Customer::new(data.id, data.name.to_owned(), data.document.to_owned())?;
        
        let id = customer.id;
        
        self.customers.insert(customer.id, customer);
        
        Ok(CustomerOutputDTO{id: data.id, name: data.name, document: data.document})
    }
}