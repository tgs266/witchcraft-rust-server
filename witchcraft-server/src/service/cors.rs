use std::str::FromStr;

// Copyright 2022 Palantir Technologies, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::service::{Layer, Service};
use http::{HeaderName, HeaderValue, Request, Response};

#[derive(Clone)]
pub struct CorsLayer {
    allow_origin: Option<String>,
    allow_credentials: bool,
    allow_methods: Option<String>,
    allow_headers: Option<String>,
}

impl CorsLayer {
    pub fn new(
        allow_origin: Option<String>,
        allow_credentials: bool,
        allow_methods: Option<String>,
        allow_headers: Option<String>,
    ) -> Self {
        CorsLayer {
            allow_origin,
            allow_credentials,
            allow_methods,
            allow_headers,
        }
    }
}

impl<S> Layer<S> for CorsLayer {
    type Service = CorsLayerService<S>;

    fn layer(self, inner: S) -> Self::Service {
        CorsLayerService {
            inner,
            allow_origin: self.allow_origin,
            allow_credentials: self.allow_credentials,
            allow_methods: self.allow_methods,
            allow_headers: self.allow_headers,
        }
    }
}

pub struct CorsLayerService<S> {
    inner: S,
    allow_origin: Option<String>,
    allow_credentials: bool,
    allow_methods: Option<String>,
    allow_headers: Option<String>,
}

impl<S, B1, B2> Service<Request<B1>> for CorsLayerService<S>
where
    S: Service<Request<B1>, Response = Response<B2>> + Sync,
    B1: Send,
{
    type Response = S::Response;

    async fn call(&self, req: Request<B1>) -> Self::Response {
        let mut response = self.inner.call(req).await;
        if let Some(allow_origin) = &self.allow_origin {
            response.headers_mut().insert(
                HeaderName::from_str("Access-Control-Allow-Origin").unwrap(),
                HeaderValue::from_str(&allow_origin).unwrap(),
            );
        }

        if let Some(allow_methods) = &self.allow_methods {
            response.headers_mut().insert(
                HeaderName::from_str("Access-Control-Allow-Methods").unwrap(),
                HeaderValue::from_str(&allow_methods).unwrap(),
            );
        }

        if let Some(allow_headers) = &self.allow_headers {
            response.headers_mut().insert(
                HeaderName::from_str("Access-Control-Allow-Headers").unwrap(),
                HeaderValue::from_str(&allow_headers).unwrap(),
            );
        }

        match self.allow_credentials {
            true => response.headers_mut().insert(
                HeaderName::from_str("Access-Control-Allow-Credentials").unwrap(),
                HeaderValue::from_str("true").unwrap(),
            ),
            false => response.headers_mut().insert(
                HeaderName::from_str("Access-Control-Allow-Credentials").unwrap(),
                HeaderValue::from_str("false").unwrap(),
            ),
        };

        response
    }
}
