use diesel::prelude::*;
use diesel::PgConnection;
use crate::models::{User,UpdateUser,Chef};
use bcrypt::{verify};
use rocket_contrib::json::{JsonValue};
use crate::auth::*;
use crate::schema;
use chrono::Local;

pub fn all_users(con:PgConnection) -> JsonValue {

    use schema::smor_users::dsl::*;

    let results =  smor_users.filter(role.eq(1)).order(id.desc()).load::<User>(&con).expect("Error unable to fetch user");
    return json!({
        "status":true,
        "data":results
    })
}
pub fn all_staffs(con:PgConnection) -> JsonValue {

    use schema::smor_users::dsl::*;

    let results =  smor_users.filter(role.eq(2).or(role.eq(3)).or(role.eq(4))).order(id.desc()).load::<User>(&con).expect("Error unable to fetch user");
    return json!({
        "status":true,
        "data":results
    })
}
pub fn all_chef_profiles(con:PgConnection) -> JsonValue {
    use schema::smor_chef_profiles::dsl::*;
    let results =  smor_chef_profiles.order(id.desc()).load::<Chef>(&con).expect("Error unable to fetch chefs profile");
    return json!({
        "status":true,
        "data":results
    })
}
pub fn login_admin(con:PgConnection,user:String,password:String) -> JsonValue {
    let clean_password = password.trim();
    let clean_email = user.trim();

    // print!(" email --> {} password --> {} ",clean_email,clean_password);

    if clean_password.is_empty() || clean_email.is_empty() {
        json!({
            "status":false,
            "message":"invalid email or password"
        })
    } else {

    use schema::smor_users::dsl::*;
    let results = smor_users.filter(email.eq(clean_email))
    .load::<User>(&con).expect("Error unable to fetch user");
        // print!("query result  {:?}",results);
    if results.is_empty() {
        json!({
            "status":false,
            "message":"invalid email or password"
        })
    } else {
        if results[0].role == 1 || !results[0].status  {
            json!({
                "status":false,
                "message":"invalid email or password"
            })
        }else{
            let verify_admin = verify(clean_password, &results[0].password);
                match verify_admin {
                    Ok(valid) => {
                        if valid {
                            let iat = Local::now().to_string();
                            let user = format!("{}{}{}",results[0].name,results[0].email,results[0].user_id).to_string();
                            let u_role =  &results[0].role.to_string();
                            let token = generate_token(&user,&iat,&u_role);

                            json!(
                                {
                                "status":true,
                                "data":{
                                    "id":results[0].id,
                                    "user_id":results[0].user_id,
                                    "name":results[0].name,
                                    "avatar":results[0].avatar,
                                    "phone":results[0].phone,
                                    "email":results[0].email,
                                    "role":results[0].role,
                                    "status":results[0].status,
                                    "token":token
                                }
                            }
                        )
                        }else{
                            json!({
                                "status":false,
                                "message":"Invalid email or password"
                            })
                        }
                            
                        
                    },
                    Err(_) => json!({
                        "status":false,
                        "message":"Invalid email or password"
                    })
                } 
        }
   
      
    }
  }
}
pub fn grant_admin_and_revoke_admin(con:PgConnection,n_user:UpdateUser) -> JsonValue {
    use schema::smor_users::dsl::*;

    diesel::update(&n_user)
                                                .set((
                                                    role.eq(&n_user.role),
                                                    update_at.eq(&n_user.update_at)
                                                ))
                                                .execute(&con)
                                                .expect("Error updating product");
    return json!({
                "status": true,
                "message":"Your request was successfully"
            })

}
pub fn ban_and_unban_user(con:PgConnection,n_user:UpdateUser) -> JsonValue {
    use schema::smor_users::dsl::*;
    diesel::update(&n_user)
                                                .set((
                                                    status.eq(&n_user.status),
                                                    update_at.eq(&n_user.update_at)
                                                ))
                                                .execute(&con)
                                                .expect("Error updating product");
    return json!({
                "status": true,
                "data":"User successfully ban"
            })
}
pub fn delete_user(con:PgConnection,uid:String) -> JsonValue {
    use schema::smor_users::dsl::*;

    diesel::delete(smor_users.filter(user_id.eq(uid)))
    .execute(&con)
        .expect("Error deleting user");

    return json!({
            "status": true,
            "data":"User account deleted successfully"
        })
}
pub fn delete_chef_profile(con:PgConnection,uid:String){
    use schema::smor_chef_profiles::dsl::*;
    diesel::delete(smor_chef_profiles.filter(user_id.eq(uid)))
    .execute(&con)
        .expect("Error deleting user chef profile");
}
