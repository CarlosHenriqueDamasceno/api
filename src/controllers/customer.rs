use rocket::serde::{Serialize, json::Json};
use api::{infra::memory_customer_repository, use_cases::customer::GetCustomer};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReturnCustomer{
    id: u32,
    name: String,
    document: String,
} 


#[get("/customers/<id>")]
pub fn get_customer(id: u32) -> Json<ReturnCustomer> {

    let customer_repository = memory_customer_repository::MemoryCustomerRepository::new();
    let get_customer = GetCustomer::new(customer_repository);
        
    match get_customer.execute(id){

        Ok(customer) => {

            let result = ReturnCustomer {
                id: customer.id,
                name: customer.name,
                document: customer.document
            };

            return Json(result);
        }
        _ => {

            panic!("error");
            
        }
    }
}