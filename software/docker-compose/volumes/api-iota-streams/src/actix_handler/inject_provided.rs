use crate::actix_handler::util::get_module_from_provider_state;
use actix_web::dev::{Payload, PayloadStream};
use actix_web::error::ErrorInternalServerError;
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::future;
use shaku::Interface;
use shaku::{HasProvider, ModuleInterface};
use std::marker::PhantomData;
use std::ops::Deref;

pub struct InjectProvided<M: ModuleInterface + HasProvider<I> + ?Sized, I: Interface + ?Sized>(
  Box<I>,
  PhantomData<M>,
);

impl<M: ModuleInterface + HasProvider<I> + ?Sized, I: Interface + ?Sized> FromRequest
  for InjectProvided<M, I>
{
  type Error = Error;
  type Future = future::Ready<Result<Self, Error>>;
  // type Config = ();

  fn from_request(req: &HttpRequest, _: &mut Payload<PayloadStream>) -> Self::Future {
    let module = match get_module_from_provider_state::<M, I>(&req) {
      Ok(module) => module,
      Err(e) => return future::err(e),
    };
    let service = match module.provide() {
      Ok(service) => service,
      Err(e) => return future::err(ErrorInternalServerError(e)),
    };

    future::ok(InjectProvided(service, PhantomData))
  }
}

impl<M: ModuleInterface + HasProvider<I> + ?Sized, I: Interface + ?Sized> Deref
  for InjectProvided<M, I>
{
  type Target = I;

  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}
