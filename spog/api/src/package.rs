use crate::error::Error;
use crate::guac::service::GuacService;
use crate::search;
use crate::server::AppState;
use crate::service::collectorist::CollectoristService;
use crate::service::v11y::V11yService;
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, HttpResponseBuilder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use bytes::BytesMut;
use csaf::definitions::ProductIdT;
use csaf::Csaf;
use futures::{stream, TryStreamExt};
use spog_model::csaf::{find_product_relations, trace_product};
use spog_model::cve::{
    AdvisoryOverview, CveDetails, CveSearchDocument, PackageRelatedToProductCve, ProductCveStatus, ProductRelatedToCve,
};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::sync::Arc;
use tracing::instrument;
use trustification_api::search::{SearchOptions, SearchResult};
use trustification_auth::authenticator::Authenticator;
use trustification_auth::client::{BearerTokenProvider, TokenProvider};
use trustification_infrastructure::new_auth;
use v11y_client::search::{SearchDocument, SearchHit};

pub(crate) fn configure(auth: Option<Arc<Authenticator>>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(
            web::scope("/api/v1/package")
                .wrap(new_auth!(auth))
                .service(web::resource("").to(package_search))
                .service(web::resource("/{id}").to(package_get)),
        );
    }
}

async fn package_search(
    web::Query(params): web::Query<search::QueryParams>,
    v11y: web::Data<V11yService>,
    state: web::Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    let SearchResult {
        result,
        total
    } = v11y.search(params).await.map_err(Error::V11y)?;

    Ok(HttpResponse::Ok().json(SearchResult { total, result }))
}

async fn package_get(id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let id = id.into_inner();

    let response = v11y.fetch(&id).await?;

    Ok(HttpResponseBuilder::new(response.status()).streaming(response.bytes_stream()))
}
