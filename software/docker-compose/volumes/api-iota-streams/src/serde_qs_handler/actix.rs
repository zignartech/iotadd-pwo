use actix_web::dev::Payload;
use actix_web::{Error as ActixError, FromRequest, HttpRequest, HttpResponse, ResponseError};
use futures::future::{ready, Ready};
use serde::de;
use serde_qs::Config as QsConfig;
use serde_qs::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

struct QsError(Error);

impl ResponseError for QsError {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::BadRequest().finish()
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct QsQuery<T>(T);

impl<T> Deref for QsQuery<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}

impl<T> DerefMut for QsQuery<T> {
  fn deref_mut(&mut self) -> &mut T {
    &mut self.0
  }
}

impl<T> QsQuery<T> {
  pub fn into_inner(self) -> T {
    self.0
  }
}

impl<T: Debug> Debug for QsQuery<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl<T: Display> Display for QsQuery<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl<T> FromRequest for QsQuery<T>
where
  T: de::DeserializeOwned,
{
  type Error = ActixError;
  type Future = Ready<Result<Self, ActixError>>;
  type Config = QsQueryConfig;

  #[inline]
  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    let query_config = req.app_data::<QsQueryConfig>();

    let error_handler = query_config.map(|c| c.ehandler.clone()).unwrap_or(None);

    let default_qsconfig = QsConfig::default();
    let qsconfig = query_config
      .map(|c| &c.qs_config)
      .unwrap_or(&default_qsconfig);

    let res = qsconfig
      .deserialize_str::<T>(req.query_string())
      .map(|val| Ok(QsQuery(val)))
      .unwrap_or_else(move |e| {
        let err = if let Some(error_handler) = error_handler {
          (error_handler)(e, req)
        } else {
          e.into()
        };

        Err(err)
      });
    ready(res)
  }
}

pub struct QsQueryConfig {
  ehandler: Option<Arc<dyn Fn(QsError, &HttpRequest) -> ActixError + Send + Sync>>,
  qs_config: QsConfig,
}

impl QsQueryConfig {
  pub fn error_handler<F>(mut self, f: F) -> Self
  where
    F: Fn(QsError, &HttpRequest) -> ActixError + Send + Sync + 'static,
  {
    self.ehandler = Some(Arc::new(f));
    self
  }

  pub fn qs_config(mut self, config: QsConfig) -> Self {
    self.qs_config = config;
    self
  }
}

impl Default for QsQueryConfig {
  fn default() -> Self {
    QsQueryConfig {
      ehandler: None,
      qs_config: QsConfig::default(),
    }
  }
}
