use std::sync::{Arc, Mutex};

use crate::request_inputs::CreateUserInput;
use crate::request_outputs::{CreateUserOutput, SigninOutput};
use poem::http::StatusCode;
use poem::{
    handler,
    web::{Data, Json},
};
use store::store::Store;

#[handler]
pub fn sign_up(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateUserOutput> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s
        .sign_up(data.username, data.password)
        .map_err(|_| Error::from_status(StatusCode::CONFLICT))?;

    let response = CreateUserOutput { id };

    Ok(Json(response))
}

#[handler]
pub fn sign_in(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<SigninOutput>, Error> {
    let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.sign_in(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            //let response = SigninOutput { jwt: user_id };
            let my_claims = Claims {
                sub: user_id,
                exp: 1111111111111,
            };
            //jwt.sign({sub:user_id},"secret")
            let token = encode(
                &Header::default(),
                &my_claims,
                &EncodingKey::from_secret(env::var("JWT_SECRET").as_ref()),
            )
            .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

            let response = SigninOutput { jwt: token };
            Ok(Json(response))
        }
        Err(_) => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
    }
}
