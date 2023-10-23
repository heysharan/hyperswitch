//! Analysis for usage of Routing in Payment flows
//!
//! Functions that are used to perform the api level configuration, retrieval, updation
//! of Routing configs.

use actix_web::{web, HttpRequest, Responder};
use api_models::routing as routing_types;
#[cfg(feature = "business_profile_routing")]
use api_models::routing::{RoutingRetrieveLinkQuery, RoutingRetrieveQuery};
use router_env::{
    tracing::{self, instrument},
    VasFlow,
};

use crate::{
    core::{api_locking, routing},
    routes::AppState,
    services::{api as oss_api, authentication as oss_auth, authentication as auth},
};

#[cfg(feature = "olap")]
#[instrument(skip_all)]
pub async fn routing_create_config(
    state: web::Data<AppState>,
    req: HttpRequest,
    json_payload: web::Json<routing_types::RoutingConfigRequest>,
) -> impl Responder {
    let flow = VasFlow::RoutingCreateConfig;
    Box::pin(oss_api::server_wrap(
        flow,
        state,
        &req,
        json_payload.into_inner(),
        |state, auth: oss_auth::AuthenticationData, payload| {
            routing::create_routing_config(
                state,
                auth.merchant_account,
                #[cfg(not(feature = "business_profile_routing"))]
                auth.key_store,
                payload,
            )
        },
        #[cfg(not(feature = "release"))]
        auth::auth_type(&oss_auth::ApiKeyAuth, &auth::JWTAuth, req.headers()),
        #[cfg(feature = "release")]
        &auth::JWTAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

#[cfg(feature = "olap")]
#[instrument(skip_all)]
pub async fn routing_link_config(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let flow = VasFlow::RoutingLinkConfig;
    Box::pin(oss_api::server_wrap(
        flow,
        state,
        &req,
        path.into_inner(),
        |state, auth: oss_auth::AuthenticationData, algorithm_id| {
            routing::link_routing_config(
                state,
                auth.merchant_account,
                #[cfg(not(feature = "business_profile_routing"))]
                auth.key_store,
                algorithm_id,
            )
        },
        #[cfg(not(feature = "release"))]
        auth::auth_type(&oss_auth::ApiKeyAuth, &auth::JWTAuth, req.headers()),
        #[cfg(feature = "release")]
        &auth::JWTAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

#[cfg(feature = "olap")]
#[instrument(skip_all)]
pub async fn routing_retrieve_config(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let algorithm_id = path.into_inner();
    let flow = VasFlow::RoutingRetrieveConfig;
    Box::pin(oss_api::server_wrap(
        flow,
        state,
        &req,
        algorithm_id,
        |state, auth: oss_auth::AuthenticationData, algorithm_id| {
            routing::retrieve_routing_config(state, auth.merchant_account, algorithm_id)
        },
        #[cfg(not(feature = "release"))]
        auth::auth_type(&oss_auth::ApiKeyAuth, &auth::JWTAuth, req.headers()),
        #[cfg(feature = "release")]
        &auth::JWTAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

#[cfg(feature = "olap")]
#[instrument(skip_all)]
pub async fn routing_retrieve_dictionary(
    state: web::Data<AppState>,
    req: HttpRequest,
    #[cfg(feature = "business_profile_routing")] query: web::Query<RoutingRetrieveQuery>,
) -> impl Responder {
    #[cfg(feature = "business_profile_routing")]
    {
        let flow = VasFlow::RoutingRetrieveDictionary;
        Box::pin(oss_api::server_wrap(
            flow,
            state,
            &req,
            query.into_inner(),
            |state, auth: oss_auth::AuthenticationData, query_params| {
                routing::retrieve_merchant_routing_dictionary(
                    state,
                    auth.merchant_account,
                    query_params,
                )
            },
            #[cfg(not(feature = "release"))]
            auth::auth_type(&oss_auth::ApiKeyAuth, &auth::JWTAuth, req.headers()),
            #[cfg(feature = "release")]
            &auth::JWTAuth,
            api_locking::LockAction::NotApplicable,
        ))
        .await
    }

    #[cfg(not(feature = "business_profile_routing"))]
    {
        let flow = VasFlow::RoutingRetrieveDictionary;
        Box::pin(oss_api::server_wrap(
            flow,
            state,
            &req,
            (),
            |state, auth: oss_auth::AuthenticationData, _| {
                routing::retrieve_merchant_routing_dictionary(state, auth.merchant_account)
            },
            #[cfg(not(feature = "release"))]
            auth::auth_type(&oss_auth::ApiKeyAuth, &auth::JWTAuth, req.headers()),
            #[cfg(feature = "release")]
            &auth::JWTAuth,
            api_locking::LockAction::NotApplicable,
        ))
        .await
    }
}
