mod car;

#[cfg(test)]
mod tests {

    use crate::car;

    #[test]
    fn get_car() {

        let car_repository = Box::new(car::MemoryCarRepository::new());
        let get_car = car::GetCar::new(car_repository);

        match get_car.execute(1){
            Ok(car) => {

                assert_eq!(car.model, String::from("Onix"));
            }
            Err(error) => {
                assert_eq!(error.to_string(), String::from("Onix"));
            }
        }
    }
}