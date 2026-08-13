//! Website and public status route handlers.
//!
//! Provides endpoints for website CRUD, incident listing, and public status pages.

use std::sync::{Arc, Mutex};

use crate::auth_middleware::UserId;
use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::{
    CreateWebsiteOutput, GetIncidentsOutput, GetMaintenancesOutput, GetWebsiteOutput,
    GetWebsitesOutput, IncidentOutput, MaintenanceOutput, PublicHistoryOutput, PublicStatusOutput,
    StatPeriodsOutput, TickOutput, WebsiteWithTicksOutput, WebsiteStatOutput, ComponentOutput,
    HistoryOutput,
};
use chrono::Utc;
use poem::{
    handler,
    web::{Data, Json, Path},
};
use store::store::Store;

/// Retrieves a single website with its recent health check ticks.
///
/// Requires authentication. Only returns the website if it belongs to the user.
#[handler]
pub async fn get_website(
    Path(id): Path<String>,
    UserId(user_id): UserId,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id.clone(), user_id).unwrap();
    let ticks = locked_s.get_ticks_for_website(website.id.clone(), 10).unwrap();

    let tick_outputs = ticks
        .into_iter()
        .map(|t| TickOutput {
            id: t.id,
            response_time_ms: t.response_time_ms,
            status: t.status,
            http_status: t.http_status,
            created_at: t.created_at,
        })
        .collect();

    Json(GetWebsiteOutput {
        url: website.url,
        id: website.id,
        user_id: website.user_id,
        ticks: tick_outputs,
    })
}

/// Creates a new website monitor.
///
/// Normalizes the URL to ensure it has an `https://` or `http://` scheme.
#[handler]
pub async fn create_website(
    UserId(user_id): UserId,
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let mut url = data.url.trim().to_string();
    if !url.starts_with("http://") && !url.starts_with("https://") {
        url = format!("https://{}", url);
    }
    let website = locked_s
        .create_website(user_id, url, None)
        .unwrap();

    let response = CreateWebsiteOutput { id: website.id };
    Json(response)
}

/// Lists all websites for the authenticated user with their latest tick.
#[handler]
pub async fn get_websites(
    UserId(user_id): UserId,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetWebsitesOutput> {
    let mut locked_s = s.lock().unwrap();
    let websites = locked_s.get_websites_for_user(user_id).unwrap();

    let mut output = Vec::new();
    for website in websites {
        let ticks = locked_s.get_ticks_for_website(website.id.clone(), 1).unwrap();
        let tick_outputs: Vec<TickOutput> = ticks
            .into_iter()
            .map(|t| TickOutput {
                id: t.id,
                response_time_ms: t.response_time_ms,
                status: t.status,
                http_status: t.http_status,
                created_at: t.created_at,
            })
            .collect();

        output.push(WebsiteWithTicksOutput {
            id: website.id,
            url: website.url,
            user_id: website.user_id,
            time_added: website.time_added,
            component: website.component,
            ticks: tick_outputs,
        });
    }

    Json(GetWebsitesOutput { websites: output })
}

/// Lists all incidents for the authenticated user's websites.
#[handler]
pub async fn get_incidents(
    UserId(user_id): UserId,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetIncidentsOutput> {
    let mut locked_s = s.lock().unwrap();
    let results = locked_s.get_incidents_for_user(user_id, 50).unwrap();

    let incidents = results
        .into_iter()
        .map(|(inc, w)| IncidentOutput {
            id: inc.id,
            website_url: w.url,
            started_at: inc.started_at,
            ended_at: inc.ended_at,
            region_id: inc.region_id,
        })
        .collect();

    Json(GetIncidentsOutput { incidents })
}

/// Builds the public status page payload for a user.
///
/// Includes websites, components, uptime stats (d1/d7/d30), incidents, and maintenances.
#[handler]
pub async fn get_public_status(
    Path(user_id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<PublicStatusOutput> {
    let mut locked_s = s.lock().unwrap();
    let websites = locked_s.get_websites_for_user(user_id.clone()).unwrap();

    let now = Utc::now().naive_utc();
    let since = |hours: i64| now - chrono::Duration::hours(hours);

    let mut website_outputs = Vec::new();
    for website in &websites {
        let ticks = locked_s.get_ticks_for_website(website.id.clone(), 24).unwrap();

        let tick_outputs: Vec<TickOutput> = ticks
            .into_iter()
            .map(|t| TickOutput {
                id: t.id,
                response_time_ms: t.response_time_ms,
                status: t.status,
                http_status: t.http_status,
                created_at: t.created_at,
            })
            .collect();

        website_outputs.push(WebsiteWithTicksOutput {
            id: website.id.clone(),
            url: website.url.clone(),
            user_id: website.user_id.clone(),
            time_added: website.time_added,
            component: website.component.clone(),
            ticks: tick_outputs,
        });
    }

    let incidents = locked_s.get_public_incidents(user_id.clone(), 10).unwrap();
    let maintenances = locked_s.get_public_maintenances(user_id.clone(), 20).unwrap();

    let website_ids: Vec<String> = websites.iter().map(|w| w.id.clone()).collect();

    let d1_stats = locked_s.get_tick_stats(website_ids.clone(), since(24)).unwrap();
    let d7_stats = locked_s.get_tick_stats(website_ids.clone(), since(24 * 7)).unwrap();
    let d30_stats = locked_s.get_tick_stats(website_ids.clone(), since(24 * 30)).unwrap();

    let to_bucket = |rows: Vec<(String, String, i64)>| {
        let mut bucket = std::collections::HashMap::new();
        for (wid, status, count) in rows {
            let entry = bucket.entry(wid).or_insert((0, 0));
            if status == "Up" {
                entry.0 += count;
            } else if status == "Down" {
                entry.1 += count;
            }
        }
        bucket
    };

    let b1 = to_bucket(d1_stats);
    let b7 = to_bucket(d7_stats);
    let b30 = to_bucket(d30_stats);

    let uptime_pct = |up: i64, down: i64| -> Option<f64> {
        let total = up + down;
        if total == 0 {
            None
        } else {
            Some((up as f64 / total as f64 * 10000.0).round() / 100.0)
        }
    };

    let website_stats: Vec<WebsiteStatOutput> = website_ids
        .iter()
        .map(|id| {
            let (u1, d1) = b1.get(id).map(|(u, d)| (*u, *d)).unwrap_or((0, 0));
            let (u7, d7) = b7.get(id).map(|(u, d)| (*u, *d)).unwrap_or((0, 0));
            let (u30, d30) = b30.get(id).map(|(u, d)| (*u, *d)).unwrap_or((0, 0));
            WebsiteStatOutput {
                website_id: id.clone(),
                periods: StatPeriodsOutput {
                    d1: uptime_pct(u1, d1),
                    d7: uptime_pct(u7, d7),
                    d30: uptime_pct(u30, d30),
                },
            }
        })
        .collect();

    let stats_map: std::collections::HashMap<String, &WebsiteStatOutput> = website_stats
        .iter()
        .map(|s| (s.website_id.clone(), s))
        .collect();

    let mut groups: std::collections::HashMap<String, Vec<&WebsiteWithTicksOutput>> =
        std::collections::HashMap::new();
    for w in &website_outputs {
        let key = w.component.clone().unwrap_or_else(|| "Uncategorized".to_string());
        groups.entry(key).or_default().push(w);
    }

    let components: Vec<ComponentOutput> = groups
        .into_iter()
        .map(|(name, group)| {
            let mut all_ticks: Vec<TickOutput> = group.iter().flat_map(|w| w.ticks.clone()).collect();
            all_ticks.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            let latest = all_ticks.first();

            let mut up_d1 = 0.0;
            let mut down_d1 = 0.0;
            let mut up_d7 = 0.0;
            let mut down_d7 = 0.0;
            let mut up_d30 = 0.0;
            let mut down_d30 = 0.0;

            for w in &group {
                if let Some(s) = stats_map.get(&w.id) {
                    if let Some(d1) = s.periods.d1 {
                        up_d1 += (d1 / 100.0 * 100.0).round() / 100.0;
                        down_d1 += ((100.0 - d1) / 100.0 * 100.0).round() / 100.0;
                    }
                    if let Some(d7) = s.periods.d7 {
                        up_d7 += (d7 / 100.0 * 100.0).round() / 100.0;
                        down_d7 += ((100.0 - d7) / 100.0 * 100.0).round() / 100.0;
                    }
                    if let Some(d30) = s.periods.d30 {
                        up_d30 += (d30 / 100.0 * 100.0).round() / 100.0;
                        down_d30 += ((100.0 - d30) / 100.0 * 100.0).round() / 100.0;
                    }
                }
            }

            let aggregate_uptime = |up: f64, down: f64| -> Option<f64> {
                let total = up + down;
                if total == 0.0 {
                    None
                } else {
                    Some((up / total * 10000.0).round() / 100.0)
                }
            };

            let status = match latest {
                Some(t) if t.status == "Up" => "Up",
                Some(t) if t.status == "Down" => "Down",
                _ => "Unknown",
            }
            .to_string();

            ComponentOutput {
                name,
                websites: group.iter().map(|w| (*w).clone()).collect(),
                stats: StatPeriodsOutput {
                    d1: aggregate_uptime(up_d1, down_d1),
                    d7: aggregate_uptime(up_d7, down_d7),
                    d30: aggregate_uptime(up_d30, down_d30),
                },
                status,
            }
        })
        .collect();

    let incident_outputs: Vec<IncidentOutput> = incidents
        .into_iter()
        .map(|(inc, w)| IncidentOutput {
            id: inc.id,
            website_url: w.url,
            started_at: inc.started_at,
            ended_at: inc.ended_at,
            region_id: inc.region_id,
        })
        .collect();

    let maintenance_outputs: Vec<MaintenanceOutput> = maintenances
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

    Json(PublicStatusOutput {
        components,
        incidents: incident_outputs,
        maintenances: maintenance_outputs,
        websites: website_outputs,
        stats: website_stats,
    })
}

/// Returns a combined timeline of incidents and maintenance events for a user.
#[handler]
pub async fn get_public_status_history(
    Path(user_id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<PublicHistoryOutput> {
    let mut locked_s = s.lock().unwrap();

    let incidents = locked_s.get_public_incidents(user_id.clone(), 100).unwrap();
    let maintenances = locked_s.get_maintenances_for_user(user_id.clone(), 100).unwrap();

    let mut history: Vec<HistoryOutput> = incidents
        .into_iter()
        .map(|(inc, w)| HistoryOutput {
            history_type: "incident".to_string(),
            id: inc.id,
            website_url: w.url,
            started_at: inc.started_at,
            ended_at: inc.ended_at,
            title: None,
            status: None,
        })
        .collect();

    for (m, w) in maintenances {
        history.push(HistoryOutput {
            history_type: "maintenance".to_string(),
            id: m.id,
            website_url: w.url,
            started_at: m.starts_at,
            ended_at: m.ends_at,
            title: Some(m.title),
            status: Some(m.status),
        });
    }

    history.sort_by(|a, b| b.started_at.cmp(&a.started_at));

    Json(PublicHistoryOutput { history })
}

/// Returns active and upcoming maintenance windows for a user's status page.
#[handler]
pub async fn get_public_maintenance(
    Path(user_id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetMaintenancesOutput> {
    let mut locked_s = s.lock().unwrap();
    let results = locked_s.get_public_maintenances(user_id, 20).unwrap();

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
