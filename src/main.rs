#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate uuid;
extern crate chrono;
extern crate jwt;
extern crate hmac;
extern crate sha2;
extern crate bcrypt;

mod schema;

mod admin_routes;
mod admin_handler;
mod user_handler;
mod user_routes;
mod models;
mod auth;

fn main() {
    models::establish_connection();
    rocket::ignite()
    .mount("/api/v1/user", routes![
        admin_routes::index,
        user_routes::r_login_user,   
        user_routes::r_register_chef,
        user_routes::r_register_user, 
        user_routes::rate_chef,
        user_routes::update_chef_bio,
        user_routes::update_profile,
        user_routes::upload_pic,
        user_routes::see_chef,
        user_routes::see_user,
        ])
    .mount("/api/v1/admin", routes![
        admin_routes::index,
        admin_routes::r_login_admin, 
        admin_routes::ban_unban_user,
        admin_routes::delete_user_account,
        admin_routes::grant_revoke_admin     
        ])
    .register(
        catchers![
            admin_routes::not_found,
            admin_routes::server_error,
            admin_routes::bad_request,
            admin_routes::unprocessable_entity,
            admin_routes::not_authorised,
            admin_routes::not_authoritative,
            admin_routes::forbidden
        ]
    )
    .launch();
}
