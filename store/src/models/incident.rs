/// Represents a downtime incident.
use crate::store::Store;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::incident)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Incident {
    pub id: String,
    pub website_id: String,
    pub region_id: String,
    pub started_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
}

impl Store {
    /// Fetches incidents for websites owned by a specific user.
    pub fn get_incidents_for_user(
        &mut self,
        user_id: String,
        limit: i64,
    ) -> Result<Vec<(Incident, crate::models::website::Website)>, diesel::result::Error> {
        use crate::schema::incident::dsl::*;
        use crate::schema::website::dsl as w;

        let results = incident
            .inner_join(crate::schema::website::table.on(w::id.eq(website_id)))
            .filter(w::user_id.eq(user_id))
            .order(started_at.desc())
            .limit(limit)
            .select((Incident::as_select(), crate::models::website::Website::as_select()))
            .load(&mut self.conn)?;

        Ok(results)
    }

    /// Fetches public incidents for a user's status page.
    pub fn get_public_incidents(
        &mut self,
        user_id: String,
        limit: i64,
    ) -> Result<Vec<(Incident, crate::models::website::Website)>, diesel::result::Error> {
        use crate::schema::incident::dsl::*;
        use crate::schema::website::dsl as w;

        let results = incident
            .inner_join(crate::schema::website::table.on(w::id.eq(website_id)))
            .filter(w::user_id.eq(user_id))
            .order(started_at.desc())
            .limit(limit)
            .select((Incident::as_select(), crate::models::website::Website::as_select()))
            .load(&mut self.conn)?;

        Ok(results)
    }

    /// Creates a new incident record.
    pub fn create_incident(
        &mut self,
        website_id: String,
        region_id: String,
    ) -> Result<Incident, diesel::result::Error> {
        let id = Uuid::new_v4();
        let incident = Incident {
            id: id.to_string(),
            website_id,
            region_id,
            started_at: chrono::Utc::now().naive_utc(),
            ended_at: None,
        };

        diesel::insert_into(crate::schema::incident::table)
            .values(&incident)
            .returning(Incident::as_returning())
            .get_result(&mut self.conn)?;

        Ok(incident)
    }
}
