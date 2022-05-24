use std::{collections::HashMap, error::Error};

// Entities
pub struct Car {
    pub id: u32,
    pub brand: String,
    pub model: String,
    pub year: u16,
    pub plate: Option<String>,
    pub price: f32
}

// Use cases
pub struct GetCar {
    repository: Box<dyn CarRepository>
}

impl GetCar {
    
    pub fn new (repository:Box<dyn CarRepository>) -> GetCar {
        GetCar { repository }
    }

    pub fn execute(&self, id:u32) -> Result<&Car, Box<dyn Error>>{
        let result = self.repository.get(id)?;
        Ok(result)
    }
}

// Domain Repositories Contracts
pub trait CarRepository {
    fn get(&self, id:u32) -> Result<&Car, &'static str>;
}


// Infra Repositories
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
