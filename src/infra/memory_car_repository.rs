use std::collections::HashMap;

use crate::{repositories::car_repository::CarRepository, entities::Car};

pub struct MemoryCarRepository{
    cars: HashMap<u32, Car>
}

impl MemoryCarRepository {
    pub fn new() -> MemoryCarRepository{

        let mut cars = HashMap::new();

        cars.insert(1, Car{
            id: 1,
            brand: String::from("Chevrolet"),
            model: String::from("Onix"),
            plate: None,
            year: 2022,
            price: 50_000.00
        });

        MemoryCarRepository { cars }
    }
}

impl CarRepository for MemoryCarRepository {
    fn get(&self, id:u32) -> Result<&Car, &'static str> {

        match self.cars.get(&id){
            Some(v) => {
                return Ok(v)
            },
            None => {
                return Err("There is no car with this id!!");
            }
        };
    }
}