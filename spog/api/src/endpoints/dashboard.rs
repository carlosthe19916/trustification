use crate::app_state::AppState;
use crate::search;
use crate::service::v11y::V11yService;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::sync::Arc;
use time::macros::format_description;
use time::OffsetDateTime;
use tracing::instrument;
use trustification_api::search::SearchOptions;
use trustification_auth::authenticator::Authenticator;
use trustification_infrastructure::new_auth;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct DashboardStatus {
    pub sbom_summary: SbomSummary,
    pub csaf_summary: CSAFSummary,
    pub cve_summary: CveSummary,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct SbomSummary {
    /// Total number of all documents
    pub total_sboms: Option<u64>,
    /// Id of last updated doc
    pub last_updated_sbom_id: Option<String>,
    /// name of last updated doc
    pub last_updated_sbom_name: Option<String>,
    /// Updated time of last updated doc
    pub last_updated_date: Option<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct CSAFSummary {
    /// Total number of all documents
    pub total_csafs: Option<u64>,
    /// Id of last updated doc
    pub last_updated_csaf_id: Option<String>,
    /// name of last updated doc
    pub last_updated_csaf_name: Option<String>,
    /// Updated time of last updated doc
    pub last_updated_date: Option<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct CveSummary {
    /// Total number of all documents
    pub total_cves: Option<u64>,
    /// Name of last updated doc
    pub last_updated_cve: Option<String>,
    /// Updated time of last updated doc
    pub last_updated_date: Option<String>,
}

pub(crate) fn configure(auth: Option<Arc<Authenticator>>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(
            web::scope("/api/v1/dashboard")
                .wrap(new_auth!(auth))
                .service(web::resource("/status").to(get_status)),
        );
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/dashboard/status",
    responses(
    (status = 200, description = "packages search was successful", body = SearchResultPackage),
    ),
    params()
)]
#[instrument(skip(state, access_token, v11y), err)]
pub async fn get_status(
    state: web::Data<AppState>,
    _params: web::Query<search::QueryParams>,
    options: web::Query<SearchOptions>,
    access_token: Option<BearerAuth>,
    v11y: web::Data<V11yService>,
) -> actix_web::Result<HttpResponse> {
    let sbom_status_result = state
        .get_sbom_status(options.clone().into_inner(), &access_token)
        .await?;
    let vex_status_result = state
        .get_vex_status(options.clone().into_inner(), &access_token)
        .await?;
    let cve_status_result = v11y.get_cve_status().await?;

    let status = DashboardStatus {
        sbom_summary: SbomSummary {
            total_sboms: sbom_status_result.total,
            last_updated_sbom_id: sbom_status_result.last_updated_sbom_id,
            last_updated_sbom_name: sbom_status_result.last_updated_sbom_name,
            last_updated_date: get_date(sbom_status_result.last_updated_date),
        },
        csaf_summary: CSAFSummary {
            total_csafs: vex_status_result.total,
            last_updated_csaf_id: vex_status_result.last_updated_vex_id,
            last_updated_csaf_name: vex_status_result.last_updated_vex_name,
            last_updated_date: get_date(vex_status_result.last_updated_date),
        },
        cve_summary: CveSummary {
            total_cves: cve_status_result.total,
            last_updated_cve: cve_status_result.last_updated_cve_id,
            last_updated_date: get_date(cve_status_result.last_updated_date),
        },
    };

    Ok(HttpResponse::Ok().json(status))
}

fn get_date(date: Option<OffsetDateTime>) -> Option<String> {
    let fmt = format_description!("[month]-[day]-[year]:[hour]:[minute]:[second] ");

    if let Some(dt) = date {
        let date = dt.date();
        Some(date.format(fmt).unwrap_or_else(|err| {
            log::info!("Failed to format date: {err}");
            date.to_string()
        }))
    } else {
        None
    }
}
