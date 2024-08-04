mod form_handler_package;
use form_handler_package::handle;

pub fn form_handler_config(conf: &mut actix_web::web::ServiceConfig) {
    conf.service(
        actix_web::web::scope("/form").service(handle), //.service(every_message_handler),
    );
}
