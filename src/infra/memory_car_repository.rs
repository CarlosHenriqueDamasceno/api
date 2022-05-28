use std::collections::HashMap;

use crate::{repositories::CarRepository, entities::car::Car};

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
    fn get(&self, id:u32) -> Result<Car, &'static str> {

        match self.cars.get(&id){
            Some(v) => {

                let result = Car{
                    id:    v.id,
                    brand: v.brand.to_owned(),
                    model: v.model.to_owned(),
                    plate: v.plate.to_owned(),
                    year:  v.year,
                    price: v.price,
                };

                return Ok(result)
            },
            None => {
                return Err("There is no car with this id!!");
            }
        };
    }
}