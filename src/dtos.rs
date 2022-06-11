pub struct CustomerInputDTO{
    pub id: u32,
    pub name: String,
    pub document: String,
}

pub struct CustomerOutputDTO{
    pub id: u32,
    pub name: String,
    pub document: String,
}

#[derive(Debug, PartialEq)]
pub struct CarMysqlOutputDTO{
    pub id: u32,
    pub brand: String,
    pub model: String,
    pub year: u16,
    pub plate: Option<String>,
    pub price: f32,
}