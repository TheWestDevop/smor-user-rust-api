use dotenv::dotenv;
use std::env;
use uuid::Uuid;
use diesel::prelude::*;
use chrono::prelude::*;
use diesel::PgConnection;
use serde::{Serialize, Deserialize};
use crate::schema::*;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading database. \n Database url is required!!! .");
    let db_connection = PgConnection::establish(&database_url).expect("error connecting to database.");
    return db_connection  
}

 #[derive(Queryable,Serialize, Deserialize,Debug)]
  pub struct User{
      pub id: i32,
      pub user_id: String,
      pub name: String,
      pub phone: String,
      pub avatar: String,
      pub email: String,
      pub password: String,
      pub role: i32,
      pub status:bool,
      pub created_at: String,
      pub update_at: String,
  } 

  
 #[derive(Insertable,Debug)]
 #[table_name="smor_users"]
  pub struct NewUser{
    pub user_id: String,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub update_at: String,
  } 
  impl NewUser {
   pub fn new(name: String, phone: String, email: String, password: String) -> NewUser {
         let created_at = Local::now().to_string();
         let update_at = Local::now().to_string();
         let user_id =  Uuid::new_v5(
             &Uuid::NAMESPACE_OID,
             format!("{}-{}-{}",name,phone,password).to_string().as_bytes()
         ).to_string();
         NewUser {
          user_id,
          name,
          phone,
          email,
          password,
          created_at,
          update_at,
        }
        
    }
}
  #[derive(Identifiable,Debug)]
   #[table_name="smor_users"]
  pub struct UpdateUser{
    pub id:i32,
    pub user_id: String,
    pub name: String,
    pub phone: String,
    pub avatar: String,
    pub email: String,
    pub role: i32,
    pub status:bool,
    pub created_at: String,
    pub update_at: String,
} 
  impl UpdateUser {
   pub fn new(id:i32,user_id:String,name: String,phone:String,avatar: String,email: String, role:i32, status:bool, created_at:String) -> UpdateUser {
         let update_at = Local::now().to_string();
         UpdateUser {
          id,
          user_id,
          name,
          phone,
          avatar,
          email,
          role,
          status,
          created_at,
          update_at,
        }
        
    }
}

#[derive(Queryable,Serialize, Deserialize,Debug)]
  pub struct Chef{
      pub id: i32,
      pub user_id: String,
      pub dish: String,
      pub details: String,
      pub rating: i32,
      pub icon: String,
      pub experience: String,
      pub state: String,
      pub lga: String,
      pub created_at: String,
      pub update_at: String,
  }
  
  #[derive(Insertable,Debug)]
  #[table_name="smor_chef_profiles"]
   pub struct NewChef{
     pub user_id: String,
     pub dish: String,
     pub details: String,
     pub icon: String,
     pub experience: String,
     pub state: String,
     pub lga: String,
     pub created_at: String,
     pub update_at: String,
   } 
   impl NewChef {
    pub fn new(user_id: String, dish: String, details: String,icon: String,experience:String,state: String,lga: String) -> NewChef {
          let created_at = Local::now().to_string();
          let update_at = Local::now().to_string();
          NewChef {
           user_id,
           dish,
           details,
           icon,
           experience,
           state,
           lga,
           created_at,
           update_at,
         }
         
     }
 }


 #[derive(Identifiable,Debug)]
 #[table_name="smor_chef_profiles"]
 pub struct UpdateChef{
  pub id: i32,
  pub user_id: String,
  pub dish: String,
  pub details: String,
  pub rating: i32,
  pub icon: String,
  pub experience: String,
  pub state: String,
  pub lga: String,
  pub created_at: String,
  pub update_at: String,
}
impl UpdateChef {
  pub fn new(id:i32,user_id:String,dish:String,details:String,rating:i32,icon: String,experience:String,state: String,lga: String,created_at:String) -> UpdateChef {
    let update_at = Local::now().to_string();
    UpdateChef {
      id,
      user_id,
      dish,
      details,
      rating,
      icon,
      experience,
      state,
      lga,
      created_at,
      update_at
    }
  }
}

pub struct Search_Chef{
   pub state: String,
   pub lga: String,
   pub dish: String
}
impl Search_Chef {
  pub fn new(state:String,lga:String,dish:String) -> Search_Chef {
    Search_Chef{
      state,
      lga,
      dish
    }
  }
}
#[derive(FromForm,Debug)]
 pub struct NewChefForm{
  pub user_id: String,
  pub dish: String,
  pub details: String,
  pub icon: String,
  pub experience: String,
  pub state: String,
  pub lga: String
}
#[derive(FromForm,Debug)]
pub struct UpdateChefForm{
  pub id: i32,
  pub user_id: String,
  pub dish: String,
  pub details: String,
  pub rating: i32,
  pub icon: String,
  pub experience: String,
  pub state: String,
  pub lga: String,
  pub created_at: String,
}
#[derive(FromForm,Debug)]
pub struct AdminLogin {
    pub email : String,
    pub password : String,
}

#[derive(FromForm,Debug)]
pub struct UserLogin {
  pub email : String,
  pub password : String,
  pub app: String,
}

#[derive(FromForm,Debug)]
pub struct UpdateUserForm{
  pub id:i32,
  pub user_id: String,
  pub name: String,
  pub phone: String,
  pub avatar: String,
  pub email: String,
  pub role: i32,
  pub status:bool,
  pub created_at: String,
  pub update_at: String,
}
#[derive(FromForm,Debug)]
pub struct NewUserForm{
  pub name: String,
  pub phone: String,
  pub email: String,
  pub password: String,
}

#[derive(FromForm,Debug)]
pub struct Search{
  pub state: String,
  pub lga: String,
  pub dish: String
}

#[derive(FromForm,Debug)]
pub struct Integer {
   pub number:i32
}
