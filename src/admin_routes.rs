
use crate::admin_handler::*;
use crate::models::*;
use rocket_contrib::json::{JsonValue};
use rocket::request::Form;
// use crate::auth;
use crate::auth::*;


#[get("/",)]
pub fn index() -> JsonValue {
    json!({
        "status":true,
        "message":"Welcome smorfarm user services"
    })
}

#[get("/all/user")]
pub fn users(_auth:NormalAdminApiKey) -> JsonValue {
    let connect = establish_connection();
   return all_users(connect);
}
#[get("/all/staff")]
pub fn staffs(_auth:NormalAdminApiKey) -> JsonValue {
    let connect = establish_connection();
   return all_staffs(connect);
}
#[get("/all/chefs/profiles")]
pub fn chefs(_auth:NormalAdminApiKey) -> JsonValue {
    let connect = establish_connection();
   return all_chef_profiles(connect);
}

#[post("/auth", data="<data>")]
pub fn r_login_admin(data:Form<AdminLogin>) -> JsonValue{
    let connect = establish_connection();
   return login_admin(connect,data.email.to_string(),data.password.to_string());
}
#[put("/add/staff", data="<data>")]
pub fn add_staff(data:Form<NewStaffForm>) -> JsonValue{
    let connect = establish_connection();
   return register_staff(connect,data.name.to_string(),data.phone.to_string(),data.avatar.to_string(),data.email.to_string(),data.password.to_string(),data.role);
}
#[patch("/update/staff", data="<data>")]
pub fn update_staff(data:Form<UpdateUserForm>,_auth:UserApiKey) -> JsonValue {
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
   return update_staff_profile(connect,profile);
}
#[put("/grant/revoke", data="<user_data>")]
pub fn grant_revoke_admin(user_data:Form<UpdateUserForm>,_auth:SuperAdminApiKey) -> JsonValue{
    let connect = establish_connection();
    let user = UpdateUser::new(user_data.id, user_data.user_id.to_string(), user_data.name.to_string(), user_data.phone.to_string(), user_data.avatar.to_string(), user_data.email.to_string(),  user_data.role, user_data.status);
   return grant_admin_and_revoke_admin(connect,user);
}

#[patch("/ban/unban", data="<user_data>")]
pub fn ban_unban_user(user_data:Form<UpdateUserForm>,_auth:SuperAdminApiKey) -> JsonValue{
    let connect = establish_connection();
    let user = UpdateUser::new(user_data.id, user_data.user_id.to_string(), user_data.name.to_string(), user_data.phone.to_string(), user_data.avatar.to_string(), user_data.email.to_string(),  user_data.role, user_data.status);
   return ban_and_unban_user(connect,user);
}

#[delete("/delete/account/<user_id>")]
pub fn delete_user_account(user_id:String,_auth:SuperAdminApiKey) -> JsonValue{
    
    let uid = &user_id;
    let connect1 = establish_connection();
    delete_chef_profile(connect1,uid.to_string());

    let connect2 = establish_connection();

    return delete_user(connect2,user_id.to_string());
}


#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": false,
        "message": "Nothing found."
    })
}
#[catch(401)]
pub fn not_authorised() -> JsonValue {
    json!({
        "status": false,
        "message": "The request requires authentication."
    })
}
#[catch(403)]
pub fn forbidden() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like you are forbidden from accessing this service, contact admin."
    })
}
#[catch(203)]
pub fn not_authoritative() -> JsonValue {
    json!({
        "status": false,
        "message": "non-authoritative token given."
    })
}
#[catch(500)]
pub fn server_error() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like we messed up."
    })
}
#[catch(400)]
pub fn bad_request() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like you send a bad request."
    })
}
#[catch(422)]
pub fn unprocessable_entity() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like you send a processable entity."
    })
}
