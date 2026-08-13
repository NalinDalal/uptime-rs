/// Represents a scheduled maintenance window.
use crate::store::Store;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::maintenance)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Maintenance {
    pub id: String,
    pub website_id: String,
    pub title: String,
    pub description: String,
    pub starts_at: NaiveDateTime,
    pub ends_at: Option<NaiveDateTime>,
    pub status: String,
    pub created_at: NaiveDateTime,
}

impl Store {
    /// Creates a new maintenance window with `scheduled` status.
    pub fn create_maintenance(
        &mut self,
        website_id: String,
        title: String,
        description: String,
        starts_at: NaiveDateTime,
        ends_at: Option<NaiveDateTime>,
    ) -> Result<Maintenance, diesel::result::Error> {
        let id = Uuid::new_v4();
        let maintenance = Maintenance {
            id: id.to_string(),
            website_id,
            title,
            description,
            starts_at,
            ends_at,
            status: "scheduled".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        };

        diesel::insert_into(crate::schema::maintenance::table)
            .values(&maintenance)
            .returning(Maintenance::as_returning())
            .get_result(&mut self.conn)?;

        Ok(maintenance)
    }

    /// Lists maintenance windows for websites owned by a user.
    pub fn get_maintenances_for_user(
        &mut self,
        user_id: String,
        limit: i64,
    ) -> Result<Vec<(Maintenance, crate::models::website::Website)>, diesel::result::Error> {
        use crate::schema::maintenance::dsl::*;
        use crate::schema::website::dsl as w;

        let results = maintenance
            .inner_join(crate::schema::website::table.on(w::id.eq(website_id)))
            .filter(w::user_id.eq(user_id))
            .order(starts_at.desc())
            .limit(limit)
            .select((Maintenance::as_select(), crate::models::website::Website::as_select()))
            .load(&mut self.conn)?;

        Ok(results)
    }

    /// Fetches active or upcoming maintenance windows for a user's status page.
    pub fn get_public_maintenances(
        &mut self,
        user_id: String,
        limit: i64,
    ) -> Result<Vec<(Maintenance, crate::models::website::Website)>, diesel::result::Error> {
        use crate::schema::maintenance::dsl::*;
        use crate::schema::website::dsl as w;

        let now = chrono::Utc::now().naive_utc();

        let results = crate::schema::maintenance::table
            .inner_join(crate::schema::website::table.on(w::id.eq(website_id)))
            .filter(w::user_id.eq(user_id))
            .filter(starts_at.le(now))
            .filter(status.eq("scheduled").or(status.eq("in_progress")))
            .order(starts_at.asc())
            .limit(limit)
            .select((Maintenance::as_select(), crate::models::website::Website::as_select()))
            .load(&mut self.conn)?;

        Ok(results)
    }

    /// Aggregates tick counts grouped by website and status for uptime statistics.
    pub fn get_tick_stats(
        &mut self,
        website_ids: Vec<String>,
        since: NaiveDateTime,
    ) -> Result<Vec<(String, String, i64)>, diesel::result::Error> {
        use crate::schema::website_tick::dsl::*;

        let results = website_tick
            .filter(website_id.eq_any(website_ids))
            .filter(createdAt.ge(since))
            .group_by((website_id, status))
            .select((website_id, status, diesel::dsl::count_star()))
            .load(&mut self.conn)?;

        Ok(results)
    }
}
