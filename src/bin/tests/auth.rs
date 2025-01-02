use std::{sync::Arc, time::Duration};

use aksono_common::app::App;
use assert_matches2::{assert_let, assert_matches};
use fake::{
    faker::{
        internet::en::{Password, Username},
        name::en::Name,
    },
    Fake,
};
use reqwest::{Client, StatusCode};
use ruma::{
    api::{
        client::{
            account::register,
            error::{ErrorBody, ErrorKind},
            session::login,
            uiaa::{AuthData, AuthFlow, AuthType, UiaaInfo, UiaaResponse, UserIdentifier},
        },
        error::{DeserializationError, FromHttpResponseError},
        IncomingRequest, IncomingResponse as _, MatrixVersion, Metadata, OutgoingRequest,
        SendAccessToken,
    },
    serde::JsonObject,
    UserId,
};
use tokio::sync::OnceCell;

mod common;

pub struct Test {
    pub app: Arc<App>,
    pub version: MatrixVersion,
    pub client: reqwest::Client,
}

impl Test {
    #[inline(always)]
    pub fn new<M>(app: Arc<App>) -> Self
    where
        M: IncomingRequest,
    {
        Self {
            app,
            version: MatrixVersion::V1_11,
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }

    #[inline(always)]
    pub fn uri(&self, metadata: Metadata) -> String {
        let path = metadata
            .history
            .stable_endpoint_for(&[self.version])
            .unwrap();

        format!(
            "http://{}:{}{}",
            self.app.config.listener.ip(),
            self.app.config.listener.port(),
            path
        )
    }

    pub async fn send<R: OutgoingRequest>(
        &self,
        request: R,
        access_token: SendAccessToken<'_>,
    ) -> reqwest::Response {
        let uri = self.uri(R::METADATA);

        tracing::info!("sending request to: {}", &uri);

        let request: axum::http::Request<Vec<u8>> = request
            .try_into_http_request(&uri, access_token, &[self.version.clone()])
            .unwrap();
        let body = request.into_body();

        self.client.post(&uri).body(body).send().await.unwrap()
    }
}

#[tokio::test]
async fn test_registration_ok() {
    let app = common::setup().await.unwrap();
    let test = Test::new::<register::v3::Request>(app);

    let (username, password): (String, String) = (Username().fake(), Password(0..32).fake());

    let mut request = register::v3::Request::new();

    request.username = Some(username.clone());
    request.password = Some(password.clone());
    request.auth = Some(AuthData::new(AuthType::Dummy.as_ref(), None, JsonObject::new()).unwrap());

    tracing::info!("with request body: {:?}", &request);
    let response = test.send(request, SendAccessToken::None).await;

    assert_matches!(response.status(), StatusCode::OK);

    let mut dummy = axum::http::Response::builder();
    *dummy.headers_mut().unwrap() = response.headers().to_owned();

    let bytes = response.bytes().await.unwrap();

    let response = register::v3::Response::try_from_http_response(axum::http::Response::new(
        dummy.body(bytes).unwrap().into_body(),
    ))
    .unwrap();

    assert_matches!(response.access_token, Some(_));

    assert_matches!(response.device_id, Some(_));

    assert_eq!(
        response.user_id,
        UserId::parse(format!("@{}:{}", &username, &test.app.config.server_name)).unwrap()
    );
}

#[tokio::test]
async fn test_registration_uiaa_ok() {
    let app = common::setup().await.unwrap();
    let test = Test::new::<register::v3::Request>(app);

    let (username, password): (String, String) = (Username().fake(), Password(0..32).fake());

    let mut request = register::v3::Request::new();

    request.username = Some(username.clone());
    request.password = Some(password.clone());

    tracing::info!("with request body: {:?}", &request);
    let response = test.send(request, SendAccessToken::None).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let json: UiaaInfo = response.json().await.unwrap();

    assert_matches!(json.completed.first(), None);

    assert_matches!(
        json.flows.first().and_then(|flow| flow.stages.first()),
        Some(&AuthType::Dummy)
    );

    assert_matches!(
        &json,
        &UiaaInfo {
            session: Some(_),
            ..
        }
    );

    let mut request = register::v3::Request::new();

    request.username = Some(username.clone());
    request.password = Some(password.clone());
    request.auth = Some(AuthData::new(AuthType::Dummy.as_ref(), None, JsonObject::new()).unwrap());

    tracing::info!("with request body: {:?}", &request);
    let response = test.send(request, SendAccessToken::None).await;

    assert_matches!(response.status(), StatusCode::OK);

    let mut dummy = axum::http::Response::builder();
    *dummy.headers_mut().unwrap() = response.headers().to_owned();

    let bytes = response.bytes().await.unwrap();

    let response = register::v3::Response::try_from_http_response(axum::http::Response::new(
        dummy.body(bytes).unwrap().into_body(),
    ))
    .unwrap();

    assert_matches!(response.access_token, Some(_));

    assert_matches!(response.device_id, Some(_));

    assert_eq!(
        response.user_id,
        UserId::parse(format!("@{}:{}", &username, &test.app.config.server_name)).unwrap()
    );
}

#[tokio::test]
async fn test_registration_with_login_ok() {
    let app = common::setup().await.unwrap();
    let test = Test::new::<register::v3::Request>(app);

    let (username, password): (String, String) = (Username().fake(), Password(0..32).fake());

    let mut request = register::v3::Request::new();

    request.username = Some(username.clone());
    request.password = Some(password.clone());
    request.auth = Some(AuthData::new(AuthType::Dummy.as_ref(), None, JsonObject::new()).unwrap());

    tracing::info!("with request body: {:?}", &request);
    let response = test.send(request, SendAccessToken::None).await;

    assert_matches!(response.status(), StatusCode::OK);

    let mut dummy = axum::http::Response::builder();
    *dummy.headers_mut().unwrap() = response.headers().to_owned();

    let bytes = response.bytes().await.unwrap();

    let response = register::v3::Response::try_from_http_response(axum::http::Response::new(
        dummy.body(bytes).unwrap().into_body(),
    ))
    .unwrap();

    assert_matches!(response.access_token, Some(_));

    assert_matches!(response.device_id, Some(_));

    assert_eq!(
        response.user_id,
        UserId::parse(format!("@{}:{}", &username, &test.app.config.server_name)).unwrap()
    );

    let request =
        login::v3::Request::new(login::v3::LoginInfo::Password(login::v3::Password::new(
            UserIdentifier::UserIdOrLocalpart(username.clone()),
            password.clone(),
        )));

    tracing::info!("with request body: {:?}", &request);
    let response = test.send(request, SendAccessToken::None).await;

    assert_matches!(response.status(), StatusCode::OK);

    let mut dummy = axum::http::Response::builder();
    *dummy.headers_mut().unwrap() = response.headers().to_owned();

    let bytes = response.bytes().await.unwrap();

    let response = login::v3::Response::try_from_http_response(axum::http::Response::new(
        dummy.body(bytes).unwrap().into_body(),
    ))
    .unwrap();

    assert_eq!(
        response.user_id,
        UserId::parse(format!("@{}:{}", &username, &test.app.config.server_name)).unwrap()
    );
}

#[tokio::test]
async fn test_registration_invalid_username() {
    let app = common::setup().await.unwrap();
    let test = Test::new::<register::v3::Request>(app);

    let (username, password): (String, String) = (format!("inv@lid"), Password(0..32).fake());

    let mut request = register::v3::Request::new();

    request.username = Some(username.clone());
    request.password = Some(password.clone());

    tracing::info!("with request body: {:?}", &request);
    let response = test.send(request, SendAccessToken::None).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let mut dummy = axum::http::Response::builder();
    *dummy.headers_mut().unwrap() = response.headers().to_owned();

    let bytes = response.bytes().await.unwrap();

    let response = register::v3::Response::try_from_http_response(axum::http::Response::new(
        dummy.body(bytes).unwrap().into_body(),
    ));

    assert_matches!(
        response,
        Err(FromHttpResponseError::Deserialization(
            DeserializationError::Json(serde_json::Error { .. })
        ))
    );
}
