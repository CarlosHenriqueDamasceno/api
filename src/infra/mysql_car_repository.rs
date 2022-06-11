use mysql::*;
use mysql::prelude::*;
use std::error::Error;
use std::{result::Result as StdResult};

use crate::repositories::CarRepository;
use crate::dtos::CarMysqlOutputDTO;
use crate::entities::car::Car;

pub struct MysqlCarRepository {

    connection: PooledConn

}

impl MysqlCarRepository {
    
    pub fn new() -> StdResult<MysqlCarRepository, Box<dyn Error>>{

        let opts = Opts::from_url("mysql://root:@localhost:3306/car_store")?;
        let pool = Pool::new(opts)?;
        let connection = pool.get_conn()?;

        Ok(MysqlCarRepository { connection })

    }
}

impl CarRepository for MysqlCarRepository {
    
    
    
    fn getAll(&mut self) -> StdResult<Vec<Car>, Box<dyn Error>>{

        let cars = self.connection
        .query_map(
            "SELECT id, brand, model, year, plate, price from cars",
            |(id, brand, model, year, plate, price)| {
                CarMysqlOutputDTO { id, brand, model, year, plate, price}
            },
        )?;

        let mut result = Vec::new();
        for car in cars {
            result.push(Car::new(car.id, car.brand, car.model, car.year, car.plate, car.price));
        }
        
       Ok(result)
    }

    fn get(&mut self, id:u32) -> StdResult<Car, &'static str> {
        todo!()
    }
}