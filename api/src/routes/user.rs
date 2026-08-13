use std::sync::{Arc, Mutex};

use bcrypt::hash;
use chrono::NaiveDateTime;
use crate::auth_middleware::UserId;
use crate::request_inputs::{CreateUserInput, CreateMaintenanceInput, UpdateWebhookInput};
use crate::request_outputs::{
    CreateUserOutput, GetMaintenancesOutput, GetWebhookOutput, MaintenanceOutput, SigninOutput,
    UpdateWebhookOutput,
};
use jsonwebtoken::{EncodingKey, Header, encode};
use poem::http::StatusCode;
use poem::{
    handler,
    web::{Data, Json},
};
use store::store::Store;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

#[handler]
pub async fn sign_up(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateUserOutput> {
    let mut locked_s = s.lock().unwrap();

    let existing = locked_s.get_user_by_username(data.username.clone());
    if existing.is_ok() {
        return Json(CreateUserOutput { id: String::new() });
    }

    let hashed_password = hash(data.password, 10).unwrap();
    let id = locked_s.sign_up(data.username, hashed_password).unwrap();

    let response = CreateUserOutput { id };

    Json(response)
}

#[handler]
pub async fn sign_in(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<SigninOutput>, poem::Error> {
    let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.sign_in(data.username.clone(), data.password.clone());

    match user_id {
        Ok(user_id) => {
            let my_claims = Claims {
                sub: user_id,
                exp: 1111111111111,
            };
            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
            let token = encode(
                &Header::default(),
                &my_claims,
                &EncodingKey::from_secret(secret.as_ref()),
            )
            .map_err(|_| poem::Error::from_status(StatusCode::UNAUTHORIZED))?;

            let response = SigninOutput { jwt: token };
            Ok(Json(response))
        }
        Err(_) => Err(poem::Error::from_status(StatusCode::UNAUTHORIZED)),
    }
}

#[handler]
pub async fn get_webhook(
    UserId(user_id): UserId,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetWebhookOutput> {
    let mut locked_s = s.lock().unwrap();
    let url = locked_s.get_webhook_url(user_id).unwrap_or(None);
    Json(GetWebhookOutput { url })
}

#[handler]
pub async fn update_webhook(
    UserId(user_id): UserId,
    Json(data): Json<UpdateWebhookInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<UpdateWebhookOutput> {
    let mut locked_s = s.lock().unwrap();
    locked_s.update_webhook_url(user_id, data.url).unwrap();
    Json(UpdateWebhookOutput { ok: true })
}

#[handler]
pub async fn create_maintenance(
    UserId(user_id): UserId,
    Json(data): Json<CreateMaintenanceInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<MaintenanceOutput>, poem::Error> {
    let mut locked_s = s.lock().unwrap();

    let _website = locked_s
        .get_website_for_maintenance(data.website_id.clone(), user_id.clone())
        .map_err(|_| poem::Error::from_status(StatusCode::CONFLICT))?;

    let starts_at = NaiveDateTime::parse_from_str(&data.starts_at, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(&data.starts_at, "%Y-%m-%d %H:%M:%S"))
        .map_err(|_| poem::Error::from_status(StatusCode::BAD_REQUEST))?;

    let ends_at = match data.ends_at {
        Some(ref e) => Some(
            NaiveDateTime::parse_from_str(e, "%Y-%m-%dT%H:%M:%S")
                .or_else(|_| NaiveDateTime::parse_from_str(e, "%Y-%m-%d %H:%M:%S"))
                .map_err(|_| poem::Error::from_status(StatusCode::BAD_REQUEST))?,
        ),
        None => None,
    };

    let maintenance = locked_s
        .create_maintenance(
            data.website_id,
            data.title,
            data.description.unwrap_or_default(),
            starts_at,
            ends_at,
        )
        .unwrap();

    let website = locked_s.get_website(maintenance.website_id.clone(), user_id).unwrap();

    Ok(Json(MaintenanceOutput {
        id: maintenance.id,
        website_url: website.url,
        title: maintenance.title,
        description: maintenance.description,
        starts_at: maintenance.starts_at,
        ends_at: maintenance.ends_at,
        status: maintenance.status,
    }))
}

#[handler]
pub async fn get_maintenances(
    UserId(user_id): UserId,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetMaintenancesOutput> {
    let mut locked_s = s.lock().unwrap();
    let results = locked_s.get_maintenances_for_user(user_id, 50).unwrap();

    let maintenances = results
        .into_iter()
        .map(|(m, w)| MaintenanceOutput {
            id: m.id,
            website_url: w.url,
            title: m.title,
            description: m.description,
            starts_at: m.starts_at,
            ends_at: m.ends_at,
            status: m.status,
        })
        .collect();

    Json(GetMaintenancesOutput { maintenances })
}
