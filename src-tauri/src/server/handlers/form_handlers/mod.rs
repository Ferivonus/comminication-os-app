mod form_handler_package;
use form_handler_package::get_all_form_pages;
use form_handler_package::hello_world;

pub fn form_handler_config(conf: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("/form")
        .service(get_all_form_pages)
        .service(hello_world);
    conf.service(scope);
}
