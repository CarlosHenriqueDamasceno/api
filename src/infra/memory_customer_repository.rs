use std::collections::HashMap;

use crate::{repositories::CustomerRepository, entities::Customer};

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
    fn get(&self, id:u32) -> Result<&Customer, &'static str> {

        match self.customers.get(&id){
            Some(v) => {
                return Ok(v)
            },
            None => {
                return Err("There is no customer with this id!!");
            }
        };
    }
}