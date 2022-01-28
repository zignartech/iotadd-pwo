use actix_web::error::ErrorInternalServerError;
use actix_web::Error;
use actix_web::HttpRequest;
use shaku::{HasComponent, HasProvider, Interface, ModuleInterface};
use std::sync::Arc;
pub fn get_module_from_component_state<
  M: ModuleInterface + HasComponent<I> + ?Sized,
  I: Interface + ?Sized,
>(
  request: &HttpRequest,
) -> Result<&M, Error> {
  request
    .app_data::<Arc<M>>()
    .map(Arc::as_ref)
    .ok_or_else(|| ErrorInternalServerError("Failed to retrieve module from state"))
}
pub fn get_module_from_provider_state<
  M: ModuleInterface + HasProvider<I> + ?Sized,
  I: Interface + ?Sized,
>(
  request: &HttpRequest,
) -> Result<&M, Error> {
  request
    .app_data::<Arc<M>>()
    .map(Arc::as_ref)
    .ok_or_else(|| ErrorInternalServerError("Failed to retrieve module from state"))
}
