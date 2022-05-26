mod repositories;
mod infra;
pub mod use_cases;
mod entities;

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::{use_cases::{car::GetCar, customer::GetCustomer}, infra};

    #[test]
    fn get_car() {

        let car_repository = infra::memory_car_repository::MemoryCarRepository::new();
        let get_car = GetCar::new(car_repository);
        let id = 1;
        match get_car.execute(id){

            Ok(car) => {

                assert_eq!(car.model, String::from("Onix"));
            }
            _ => {

                assert!(false);
            }
        }
        
    }

    #[test]
    fn get_customer() {

        let customer_repository = infra::memory_customer_repository::MemoryCustomerRepository::new();
        let get_customer = GetCustomer::new(customer_repository);
        let id = 1;
        match get_customer.execute(id){

            Ok(customer) => {

                assert_eq!(customer.name, String::from("Carlos Henrique"));
            }
            _ => {

                assert!(false);
            }
        }
        
    }

    #[test]
    fn save_customer() {

        let customer_repository = infra::memory_customer_repository::MemoryCustomerRepository::new();

        let customer_data = HashMap::new();

        customer_data.insert("name", "Andreina");
        customer_data.insert("name", "Andreina");

        let get_customer = SaveCustomer::new(customer_repository, vec![]);
        let id = 1;
        match get_customer.execute(id){

            Ok(customer) => {

                assert_eq!(customer.name, String::from("Carlos Henrique"));
            }
            _ => {

                assert!(false);
            }
        }
        
    }
}