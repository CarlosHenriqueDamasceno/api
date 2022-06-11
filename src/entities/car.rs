#[derive(Debug)]
pub struct Car {
    pub id: u32,
    pub brand: String,
    pub model: String,
    pub year: u16,
    pub plate: Option<String>,
    pub price: f32,
}

impl Car {
    pub fn new(id: u32, brand: String, model: String, year: u16, plate: Option<String>, price: f32) -> Car{
        Car{id, brand, model, year, plate, price}
    }
}