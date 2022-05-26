use crate::repositories::CarRepository;
use crate::entities::Car;
use std::error::Error;

pub struct GetCar<T> where T: CarRepository {
    repository: T
}

impl<T> GetCar<T> where T: CarRepository{
    
    pub fn execute(&self, id:u32) -> Result<&Car, Box<dyn Error>>{
        let result = self.repository.get(id)?;
        Ok(result)
    }

    pub fn new (repository:T) -> GetCar<T> {
        GetCar { repository }
    }
}
