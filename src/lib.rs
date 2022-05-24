mod repositories;
mod infra;
pub mod use_cases;
mod entities;

#[cfg(test)]
mod tests {

    use crate::{use_cases::get_car::GetCar, infra};

    #[test]
    fn get_car() {

        let car_repository = Box::new(infra::memory_car_repository::MemoryCarRepository::new());
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
}