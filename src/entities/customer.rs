pub struct Customer{
    pub id: u32,
    pub name: String,
    pub document: String,
}

impl Customer {
    pub fn new(id: u32, name: String, document: String) -> Result<Customer, &'static str>{

        if document.chars().count() != 11 && document.chars().count() != 14 {

            return Err("Invalid document!");

        }

        Ok(Customer { id, name, document })
    } 
}