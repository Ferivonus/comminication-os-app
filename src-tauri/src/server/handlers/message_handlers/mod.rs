mod message_contact_handlers;
mod message_get_set_handlers;
mod message_handler_package;

use message_contact_handlers::add_contact_my_client;
use message_contact_handlers::add_contact_other_client;
use message_contact_handlers::get_connected_people_handler;
use message_contact_handlers::get_connecting_people_handler;
use message_get_set_handlers::get_messages_my_client;
use message_get_set_handlers::get_messages_other_client;
use message_get_set_handlers::send_message_my_client;
use message_get_set_handlers::send_message_other_client;
use message_handler_package::reset_connected_people_table_handler;
use message_handler_package::reset_connecting_people_table_handler;
use message_handler_package::reset_messages_send_to_my_client_table_handler;
use message_handler_package::reset_messages_send_to_other_client_table_handler;

pub fn message_handler_config(conf: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("/message")
        .service(reset_messages_send_to_my_client_table_handler)
        .service(reset_messages_send_to_other_client_table_handler)
        .service(send_message_my_client)
        .service(send_message_other_client)
        .service(get_messages_my_client)
        .service(get_messages_other_client)
        .service(reset_connecting_people_table_handler)
        .service(reset_connected_people_table_handler)
        .service(get_connected_people_handler)
        .service(add_contact_my_client)
        .service(add_contact_other_client)
        .service(get_connecting_people_handler);
    conf.service(scope);
}
