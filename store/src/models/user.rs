use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: String,
    username: String,
    password: String,
}
//3. implementing the db via diesel
//4. add a sign_up and sign_in function to handle with db

impl Store {
    pub fn sign_up(
        &mut self,
        username: String,
        password: String,
    ) -> Result<String, diesel::result::Error> {
        let id = Uuid::new_v4(); //generate random id
        let u = User {
            //put it into struct object
            username,
            password,
            id: id.to_string(),
        };
        diesel::insert_into(crate::schema::user::table)
            .values(&u)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;
        //insert into table, if error return error
        //else put into table
        Ok(id.to_string())
    }
    pub fn sign_in(
        &mut self,
        input_username: String,
        input_password: String,
    ) -> Result<bool, diesel::result::Error> {
        use crate::schema::user::dsl::*;

        let user_result = user //get user from table
            .filter(username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;

        if user_result.password != input_password {
            //check if password is correct
            return Ok(false);
        }

        Ok(true)
    }
}
