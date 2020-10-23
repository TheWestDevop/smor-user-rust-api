use crate::user_handler::*;
use crate::models::*;
use crate::auth::*;

use rocket_contrib::json::{JsonValue};
use rocket::request::Form;


#[post("/auth", data="<data>")]
pub fn r_login_user(data:Form<UserLogin>) -> JsonValue {
    let connect = establish_connection();
   return login_user(connect,data.email.to_string(),data.password.to_string(),data.app.to_string());
}

#[put("/register", data="<data>")]
pub fn r_register_user(data:Form<NewUserForm>) -> JsonValue {
    let connect = establish_connection();
   return register_user(connect,data.name.to_string(),data.phone.to_string(),data.email.to_string(),data.password.to_string());
}

#[patch("/register/chef", data="<data>")]
pub fn r_register_chef(data:Form<NewChefForm>,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
    let  chef = NewChef::new(
        data.user_id.to_string(),
        data.nickname.to_string(),
        data.dish.to_string(),
        data.dish_cost.to_string(),
        data.details.to_string(),
        data.icon.to_string(),
        data.experience.to_string(),
        data.state.to_string(),
        data.lga.to_string(),
        data.next_of_kin_full_name.to_string(),
        data.next_of_kin_address.to_string(),
        data.next_of_kin_phone.to_string(),
        data.next_of_kin_relationship.to_string(),

    );
   return register_chef_detail(connect,chef);
}

#[put("/update/profile", data="<data>")]
pub fn update_profile(data:Form<UpdateUserForm>,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
    let  profile = UpdateUser::new(
                                    data.id,
                                    data.user_id.to_string(),
                                    data.name.to_string(),
                                    data.phone.to_string(),
                                    data.avatar.to_string(),
                                    data.email.to_string(),
                                    data.role,
                                    data.status,
                                );
   return update_user_profile(connect,profile);
}

#[put("/update/chef/profile", data="<data>")]
pub fn update_chef_bio(data:Form<UpdateChefForm>,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
    let  profile = UpdateChef::new(
                                    data.user_id.to_string(),
                                    data.nickname.to_string(),
                                    data.dish.to_string(),
                                    data.details.to_string(),
                                    data.rating,
                                    data.icon.to_string(),
                                    data.experience.to_string(),
                                    data.state.to_string(),
                                    data.lga.to_string(),
                                );
   return update_chef_profile(connect,profile);
}

#[put("/rate/chef/<uid>",data="<rating>")]
pub fn rate_chef(uid:String,rating:Form<Integer>,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return rating_chef(connect,rating.number,uid);
}
#[get("/chef/<uid>")]
pub fn see_chef(uid:String,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return get_chef(connect,uid);
}

#[get("/profile/<uid>")]
pub fn see_user(uid:String,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return get_user(connect,uid);
}

#[put("/upload/pic/<uid>/<url>")]
pub fn upload_pic(uid:String,url:String,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return update_user_avatar(connect,url,uid);
}

#[post("/search/chef", data="<query>")]
pub fn search_chef(query:Form<Search>,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
    let data = Search_Chef::new(query.state.to_string(), query.lga.to_string(), query.dish.to_string());
   return search(connect,data);
}
#[get("/search/chef/<nick_name>")]
pub fn search_chef_nickname(nick_name:String,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return search_by_nickname(connect,nick_name);
}
#[get("/change/chef/<user_id>/status/<status>")]
pub fn availability_status(user_id:String,status:bool,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return disable_enable_availability(connect,user_id,status);
}