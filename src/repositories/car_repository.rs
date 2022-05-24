use crate::entities::Car;
pub trait CarRepository {
    fn get(&self, id:u32) -> Result<&Car, &'static str>;
}

