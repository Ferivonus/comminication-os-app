mod message_handler_package;

use message_handler_package::create_messages_send_to_my_client_table_handler;
use message_handler_package::create_messages_send_to_other_client_table_handler;
use message_handler_package::get_messages_my_client;
use message_handler_package::get_messages_other_client;
use message_handler_package::send_message_my_client;
use message_handler_package::send_message_other_client;

pub fn message_handler_config(conf: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("/message")
        .service(create_messages_send_to_my_client_table_handler)
        .service(create_messages_send_to_other_client_table_handler)
        .service(send_message_my_client)
        .service(send_message_other_client)
        .service(get_messages_my_client)
        .service(get_messages_other_client);
    conf.service(scope);
}
