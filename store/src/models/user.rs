/// Represents a user account in the database.
use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub webhook_url: Option<String>,
}

impl Store {
    /// Looks up a user by their unique username.
    pub fn get_user_by_username(
        &mut self,
        input_username: String,
    ) -> Result<User, diesel::result::Error> {
        use crate::schema::user::dsl::*;

        let user_result = user
            .filter(username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;

        Ok(user_result)
    }

    /// Creates a new user account.
    ///
    /// Returns the generated UUID for the new user.
    pub fn sign_up(
        &mut self,
        username: String,
        password: String,
    ) -> Result<String, diesel::result::Error> {
        let id = Uuid::new_v4();
        let u = User {
            id: id.to_string(),
            username,
            password,
            webhook_url: None,
        };
        diesel::insert_into(crate::schema::user::table)
            .values(&u)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;
        Ok(id.to_string())
    }

    /// Authenticates a user by username and password.
    ///
    /// Returns the user's ID on success, or `Error::NotFound` if credentials
    /// do not match.
    pub fn sign_in(
        &mut self,
        input_username: String,
        input_password: String,
    ) -> Result<String, diesel::result::Error> {
        use crate::schema::user::dsl::*;

        let user_result = user
            .filter(username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;

        if user_result.password != input_password {
            return Err(diesel::result::Error::NotFound);
        }

        Ok(user_result.id)
    }

    /// Retrieves the stored webhook URL for a user, if set.
    pub fn get_webhook_url(
        &mut self,
        input_user_id: String,
    ) -> Result<Option<String>, diesel::result::Error> {
        use crate::schema::user::dsl::*;

        let result = user
            .filter(id.eq(input_user_id))
            .select(webhook_url)
            .first(&mut self.conn)?;

        Ok(result)
    }

    /// Updates the webhook URL for a user.
    pub fn update_webhook_url(
        &mut self,
        input_user_id: String,
        url: String,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::user::dsl::*;

        diesel::update(user)
            .filter(id.eq(input_user_id))
            .set(webhook_url.eq(url))
            .execute(&mut self.conn)?;

        Ok(())
    }
}
