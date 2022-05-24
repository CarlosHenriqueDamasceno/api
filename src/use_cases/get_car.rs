use crate::repositories::car_repository::CarRepository;
use crate::entities::Car;
use std::error::Error;
pub struct GetCar {
    repository: Box<dyn CarRepository>
}

impl GetCar {
    
    pub fn execute(&self, id:u32) -> Result<&Car, Box<dyn Error>>{
        let result = self.repository.get(id)?;
        Ok(result)
    }

    pub fn new (repository:Box<dyn CarRepository>) -> GetCar {
        GetCar { repository }
    }
}