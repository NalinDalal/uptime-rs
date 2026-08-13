/// Represents a monitored website and related database records.
use crate::store::Store;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

/// A monitored website.
#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub user_id: String,
    pub time_added: NaiveDateTime,
    pub component: Option<String>,
}

/// A single health check tick for a website.
#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website_tick)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WebsiteTick {
    pub id: String,
    pub response_time_ms: i32,
    pub status: String,
    pub http_status: Option<i32>,
    pub region_id: String,
    pub website_id: String,
    #[diesel(column_name = createdAt)]
    pub created_at: NaiveDateTime,
}

/// A downtime incident.
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

/// A scheduled or active maintenance window.
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
    /// Creates a new monitored website for a user.
    pub fn create_website(
        &mut self,
        user_id: String,
        url: String,
        component: Option<String>,
    ) -> Result<Website, diesel::result::Error> {
        let id = Uuid::new_v4();
        let website = Website {
            user_id,
            url,
            id: id.to_string(),
            time_added: chrono::Utc::now().naive_utc(),
            component,
        };

        let website = diesel::insert_into(crate::schema::website::table)
            .values(&website)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)?;

        Ok(website)
    }

    /// Retrieves a single website by its ID, ensuring it belongs to the given user.
    pub fn get_website(
        &mut self,
        input_id: String,
        input_user_id: String,
    ) -> Result<Website, diesel::result::Error> {
        use crate::schema::website::dsl::*;

        let website_result = website
            .filter(id.eq(input_id))
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .first(&mut self.conn)?;

        Ok(website_result)
    }

    /// Lists all websites belonging to a user.
    pub fn get_websites_for_user(
        &mut self,
        input_user_id: String,
    ) -> Result<Vec<Website>, diesel::result::Error> {
        use crate::schema::website::dsl::*;

        let results = website
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .load(&mut self.conn)?;

        Ok(results)
    }

    /// Fetches recent health check ticks for a website.
    pub fn get_ticks_for_website(
        &mut self,
        website_id: String,
        limit: i64,
    ) -> Result<Vec<WebsiteTick>, diesel::result::Error> {
        use crate::schema::website_tick::dsl::*;

        let results = website_tick
            .filter(website_id.eq(website_id))
            .order(createdAt.desc())
            .limit(limit)
            .select(WebsiteTick::as_select())
            .load(&mut self.conn)?;

        Ok(results)
    }

    /// Verifies that a website belongs to a user before creating maintenance.
    pub fn get_website_for_maintenance(
        &mut self,
        website_id: String,
        user_id: String,
    ) -> Result<Website, diesel::result::Error> {
        use crate::schema::website::dsl::*;

        let website_result = website
            .filter(id.eq(website_id))
            .filter(user_id.eq(user_id))
            .select(Website::as_select())
            .first(&mut self.conn)?;

        Ok(website_result)
    }
}
