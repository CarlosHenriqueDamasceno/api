mod repositories;
mod entities;
mod dtos;
pub mod infra;
pub mod use_cases;
// mod controllers;


#[cfg(test)]
mod tests {

    use crate::{use_cases::{car::GetCar, get_customer::GetCustomer, save_customer::SaveCustomer}, infra, dtos::CustomerInputDTO};

    #[test]
    fn get_car() {

        let car_repository  = infra::memory_car_repository::MemoryCarRepository::new();
        let get_car = GetCar::new(car_repository);
        let id                              = 1;
        
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

        let customer_repository       = infra::memory_customer_repository::MemoryCustomerRepository::new();
        let get_customer = GetCustomer::new(customer_repository);
        let id                                             = 1;
        
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

        let customer_data = CustomerInputDTO{id: 2, name: String::from("Andreina"), document: String::from("01547896542")};
        let mut customer_repository = infra::memory_customer_repository::MemoryCustomerRepository::new();


        let mut save_customer = SaveCustomer::new(&mut customer_repository);
        match save_customer.execute(customer_data){

            Ok(result) => {

                let get_customer = GetCustomer::new(customer_repository);

                let test = get_customer.execute(result.id).unwrap(); 

                assert_eq!(test.name, String::from("Andreina"));
            }
            _ => {

                assert!(false);
            }
        }
        
    }


}