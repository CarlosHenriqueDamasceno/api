pub struct Car {
    pub id: u32,
    pub brand: String,
    pub model: String,
    pub year: u16,
    pub plate: Option<String>,
    pub price: f32,
}

pub struct Customer{
    pub id: u32,
    pub name: String,
    pub document: String,
}

pub struct Sale {

    pub id: u32,
    pub customer_id: u32,
    pub car_id: u32,
    pub payment_method: String,

}