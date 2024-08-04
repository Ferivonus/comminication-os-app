mod wailing_handler_package;
use wailing_handler_package::handle;

pub fn message_handler_config(conf: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("/wailing").service(handle);
    conf.service(scope);
}
