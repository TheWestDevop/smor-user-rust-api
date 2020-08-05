use diesel::prelude::*;
use diesel::PgConnection;
use crate::models::*;
use bcrypt::{DEFAULT_COST, hash,verify};
use rocket_contrib::json::{JsonValue};
 use crate::auth::*;
use crate::schema;
use chrono::Local;



pub fn login_user(con:PgConnection,user:String,password:String,app:String) -> JsonValue{
    let clean_password = password.trim();
    let clean_email = user.trim();
    let clean_app = app.trim().to_lowercase();

    if clean_password.is_empty() || clean_email.is_empty() || clean_app.is_empty() {
        json!({
            "status":false,
            "message":"invalid email or password"
        })
    }else{
    use schema::smor_users::dsl::*;
    let results = smor_users.filter(email.eq(clean_email).and(role.eq(1)).or(role.eq(2)).or(role.eq(3)).and(status.eq(true)))
    .load::<User>(&con).expect("Error unable to fetch user");
    if results.is_empty() {
        json!({
            "status":false,
            "message":"invalid email or password"
        })
    } else {
      let verify_admin = verify(clean_password, &results[0].password);
      match verify_admin {
          Ok(valid) => {
               match valid {
                true => {
                        let iat = Local::now().to_string();
                        let user = format!("{}{}{}",results[0].name,results[0].email,results[0].user_id).to_string();
                        let u_role =  &results[0].role.to_string();
                        let token = generate_token(&user,&iat,&u_role);
                        match clean_app.as_str() {
                            "user" => {
                                json!(
                                    {
                                    "status":true,
                                    "data":{
                                        "user_id":results[0].user_id,
                                        "name":results[0].name,
                                        "avatar":results[0].avatar,
                                        "phone":results[0].phone,
                                        "email":results[0].email,
                                        "token":token
                                    }
                                }
                            )
                            },
                            "chef" =>  {
                                use schema::smor_chef_profiles::dsl::*;
                                let profile_result = smor_chef_profiles.filter(user_id.eq(&results[0].user_id))
                                .load::<Chef>(&con).expect("Error unable to fetch chef profile");
                                if profile_result.is_empty() {
                                    json!(
                                        {
                                        "status":false,
                                        "data":{
                                            "id":results[0].id,
                                            "user_id":results[0].user_id,
                                            "name":results[0].name,
                                            "avatar":results[0].avatar,
                                            "phone":results[0].phone,
                                            "email":results[0].email,
                                            "role":results[0].role,
                                            "status":results[0].status,
                                            "token":token,
                                            "details":"",
                                            "rating":"",
                                            "experience":"",
                                            "dish":""
                                        }
                                    }
                                )
                                } else {
                                    json!(
                                        {
                                        "status":false,
                                        "data":{
                                            "id":results[0].id,
                                            "user_id":results[0].user_id,
                                            "name":results[0].name,
                                            "avatar":results[0].avatar,
                                            "phone":results[0].phone,
                                            "email":results[0].email,
                                            "role":results[0].role,
                                            "status":results[0].status,
                                            "token":token,
                                            "details":profile_result[0].details,
                                            "rating":profile_result[0].rating,
                                            "experience":profile_result[0].experience,
                                            "dish":profile_result[0].dish
                                        }
                                    }
                                    )
                                }
                            },
                            _ => json!({
                                "status":false,
                                "message":"Invalid app specified"
                            })
                        }
                 }, 
                false => json!({
                                    "status":false,
                                    "message":"Invalid email or password"
                                }),
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

pub fn register_user(con:PgConnection,n_name:String,n_phone:String,n_email:String,pass:String)-> JsonValue {
    use schema::smor_users::dsl::*;
    let n_password =  hash(pass, DEFAULT_COST).unwrap();
    let results = smor_users.filter(email.eq(&n_email))
    .load::<User>(&con).expect("Error unable to fetch user by email");
    if results.len().eq(&0) {
        use schema::smor_users;
        let new_user = NewUser::new(n_name, n_phone, n_email, n_password);
        let result = diesel::insert_into(smor_users::table)
                                                .values(new_user)
                                                .get_result::<User>(&con)
                                                .expect("Error creating new user"); 
        json!({
             "status":true,
             "data":result
            })
    }else{
        json!({
            "status":false,
            "message":"an account exist with given email"
           })
    }  
}

pub fn register_chef_detail(con:PgConnection,chef:NewChef) -> JsonValue {
    use schema::smor_chef_profiles;
    let result = diesel::insert_into(smor_chef_profiles::table)
                            .values(chef)
                            .get_result::<Chef>(&con)
                            .expect("Error creating new chef profile");
     json!({
         "status":true,
         "data":result
     })                       
}

pub fn update_user_profile(con:PgConnection,user:UpdateUser) -> JsonValue {
    use schema::smor_users::dsl::*;

    let results = diesel::update(&user)
                                                .set((
                                                    name.eq(&user.name),
                                                    phone.eq(&user.phone),
                                                    email.eq(&user.email),
                                                    update_at.eq(&user.update_at)
                                                ))
                                                .get_result::<User>(&con)
                                                .expect("Error updating user profile");
    json!({
                "status": true,
                "data":results
            })
}

pub fn update_user_avatar(con:PgConnection,url:String,uid:String) -> JsonValue {
    use schema::smor_users::dsl::*;

   diesel::update(smor_users.filter(user_id.eq(&uid)))
                                                .set(
                                                    avatar.eq(&url),
                                                )
                                                .execute(&con)
                                                .expect("Error updating user profile");
    json!({
                "status": true,
                "data":"Profile Picture Uploaded successfully"
            })
}

pub fn update_chef_profile(con:PgConnection,chef:UpdateChef)-> JsonValue {
    use schema::smor_chef_profiles::dsl::*;
    let results = diesel::update(&chef)
                                                .set((
                                                    dish.eq(&chef.dish),
                                                    details.eq(&chef.details),
                                                    icon.eq(&chef.icon),
                                                    experience.eq(&chef.experience),
                                                    state.eq(&chef.state),
                                                    lga.eq(&chef.lga),
                                                    update_at.eq(&chef.update_at)
                                                ))
                                                .get_result::<Chef>(&con)
                                                .expect("Error updating chef profile");
    json!({
                "status": true,
                "data":results
            })
}

pub fn rating_chef(con:PgConnection,n_rating:i32,uid:String) -> JsonValue {
    // use schema::smor_users::dsl::*;
    use schema::smor_chef_profiles::dsl::*;

    let p_rating = smor_chef_profiles.select(rating).filter(user_id.eq(&uid))
                                                .load::<i32>(&con).expect("Error unable to fetch chef profile for rating");
    let rate =   0.01 * n_rating as f32;
    let new_rating = rate + p_rating[0] as f32;
    diesel::update(smor_chef_profiles.filter(user_id.eq(&uid)))
                                                .set(
                                                    rating.eq(new_rating as i32),
                                                )
                                                .execute(&con)
                                                .expect("Error updating user profile");
    json!({
                "status": true,
                "data":"Chef rating was successfully"
            })
}

pub fn get_user(con:PgConnection,uid:String) -> JsonValue {
    use schema::smor_users::dsl::*;

    let results = smor_users.filter(user_id.eq(&uid))
    .load::<User>(&con).expect("Error unable to fetch user by user_id");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results[0]
    })
}
pub fn get_chef(con:PgConnection,uid:String) -> JsonValue {
    use schema::smor_users::dsl::*;
    let results = smor_users.filter(schema::smor_users::dsl::user_id.eq(&uid)).load::<User>(&con).expect("Error unable to fetch user by user_id");
    // print!("query result  {:?}",results);
    use schema::smor_chef_profiles::dsl::*;
    let profile_result = smor_chef_profiles.filter(schema::smor_chef_profiles::dsl::user_id.eq(&uid))
    .load::<Chef>(&con).expect("Error unable to fetch chef profile for rating");
    return json!(
        {
        "status":false,
        "data":{
            "id":results[0].id,
            "user_id":results[0].user_id,
            "name":results[0].name,
            "avatar":results[0].avatar,
            "phone":results[0].phone,
            "email":results[0].email,
            "role":results[0].role,
            "details":profile_result[0].details,
            "rating":profile_result[0].rating,
            "experience":profile_result[0].experience,
            "dish":profile_result[0].dish
        }
    }
    )
}

pub fn search(con:PgConnection,search:Search_Chef) -> JsonValue {
    use schema::smor_chef_profiles::dsl::*;
    let results = smor_chef_profiles.filter(state.eq(&search.state).and(lga.eq(&search.lga)).and(dish.ilike(&search.dish)))
    .load::<Chef>(&con).expect("Error unable to fetch searched dish");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    });
}
