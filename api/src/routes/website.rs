use std::sync::{Arc, Mutex};

use crate::auth_middleware::UserId;
use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::{CreateWebsiteOutput, GetWebsiteOutput};
use poem::{
    handler,
    web::{Data, Json, Path},
};
use store::schema::website::user_id;
use store::store::Store;

//2. serde and json input oputput
#[handler]
pub fn get_website(
    Path(id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserId(user_id): UserId,
) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id, user_id).unwrap();
    Json(GetWebsiteOutput {
        url: website.url,
        id: website.id,
        user_id: website.user_id,
    })
}

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserId(user_id): UserId,
) -> Json<CreateWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s
        .create_website(
            id, user_id, //String::from("dd020379-1e62-44b2-8a3d-c4e17c30d044"),
        )
        .unwrap();

    let response = CreateWebsiteOutput { id: website.id };
    Json(response)
}
