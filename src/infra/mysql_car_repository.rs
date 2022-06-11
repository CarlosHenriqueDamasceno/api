use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq)]
struct Car {
    id: u32,
    brand: String,
    model: String,
    year: u16,
    plate: Option<String>,
    price: f64,
}


struct mysqlCarRepository {

    connection: PooledConn

}

impl mysqlCarRepository {
    
    pub fn new() -> Result<mysqlCarRepository>{

        let opts = Opts::from_url("mysql://root:password@localhost:3306/car_store")?;
        let pool = Pool::new(opts)?;
        let connection = pool.get_conn()?;

        Ok(mysqlCarRepository { connection })

    }

    pub fn getAllCars() -> Vec<Car>{
        
        vec![]
    }
}