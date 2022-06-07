use rocket::{serde::{Serialize, json::{json, Value}}, Request};
use rocket::response::content;
use api::{infra::memory_customer_repository, use_cases::get_customer::GetCustomer};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReturnCustomer{
    id: u32,
    name: String,
    document: String,
} 


#[get("/customers/<id>")]
pub fn get_customer(id: u32) -> Option<Value> {

    let customer_repository = memory_customer_repository::MemoryCustomerRepository::new();
    let get_customer = GetCustomer::new(customer_repository);
        
    match get_customer.execute(id){

        Ok(customer) => {

            let result = ReturnCustomer {
                id: customer.id,
                name: customer.name,
                document: customer.document
            };

            Some(json!({ "Customer": result, }))
        }
        _ => None
    }
}


#[catch(404)]
pub fn not_found(req: &Request<'_>) -> content::RawJson<&'static str> {
    content::RawJson(r#"
                        { 
                            "error": {
                                "type": "Resource not found"
                            } 
                        }
                    "#)
}