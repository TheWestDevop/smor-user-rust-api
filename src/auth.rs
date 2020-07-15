
use rocket::{Outcome, request::{self,Request,FromRequest}, http::Status};
use hmac::{Hmac, NewMac};
use jwt::*;
use sha2::Sha256;
use std::collections::BTreeMap;






#[derive(Debug)]
pub struct  SuperAdminApiKey (pub BTreeMap<String, String>);
pub struct  NormalAdminApiKey (pub BTreeMap<String, String>);
pub struct  UserApiKey (pub BTreeMap<String, String>);


pub fn verify_token(token:&str) -> Result<BTreeMap<String, String>,Error>{
    // type Error  = !;
    let key: Hmac<Sha256> = Hmac::new_varkey(b"mysecret").unwrap();
    let token_claims: BTreeMap<String, String> = token.verify_with_key(&key)?;
    // Err(Error::NoClaimsComponent);
    // println!("token sub is {} and company is {}",token_claims["sub"].to_string(),token_claims["company"].to_string() );
    if token_claims["company"].eq("smor_group") {
        Ok(token_claims)
    }else{
       Err(Error::NoClaimsComponent,)
    }
}

pub fn generate_token(user:&str,iat:&str,role:&str) -> String {
    let key: Hmac<Sha256> = Hmac::new_varkey(b"mysecret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", user);
    claims.insert("iat",iat);
    claims.insert("role",role);
    claims.insert("company", "smor_group");

    let token = claims.sign_with_key(&key).unwrap();
    return token;
}

impl<'a, 'r> FromRequest<'a, 'r> for SuperAdminApiKey {
type Error = ();

fn from_request(request: &'a Request<'r>) -> request::Outcome<SuperAdminApiKey, ()>{
  
    let keys: Vec<_> = request.headers().get("authorization").collect();


    // print!("request header ---> {:#?}",request);

    // let nkey = ;
   
     if keys.is_empty() || keys.len() != 1  || keys[0].trim().is_empty() { 
       return  Outcome::Failure((Status::Unauthorized,()));
     }else {
     match verify_token(keys[0]) {
        Ok(claim) => {
            match claim["role"].as_str() {
             "1" => Outcome::Failure((Status::Forbidden,())),
             "2" => Outcome::Failure((Status::NonAuthoritativeInformation,())),
             "3" => Outcome::Success(SuperAdminApiKey(claim)),
             _ => Outcome::Failure((Status::NonAuthoritativeInformation,()))
            }
            
        },
        Err(_) => Outcome::Failure((Status::NonAuthoritativeInformation,())),
        // _ => Outcome::Failure((Status::NonAuthoritativeInformation,())),
    } 
  }
    // return  keys;
 }
}
impl<'a, 'r> FromRequest<'a, 'r> for NormalAdminApiKey {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> request::Outcome<NormalAdminApiKey, ()>{
      
        let keys: Vec<_> = request.headers().get("authorization").collect();
    
    
        // print!("request header ---> {:#?}",request);
    
        // let nkey = ;
       
         if keys.is_empty() || keys.len() != 1  || keys[0].trim().is_empty() { 
           return  Outcome::Failure((Status::Unauthorized,()));
         }else {
         match verify_token(keys[0]) {
            Ok(claim) => {
                match claim["role"].as_str() {
                 "1" => Outcome::Failure((Status::Forbidden,())),
                 "2" => Outcome::Success(NormalAdminApiKey(claim)),
                 "3" => Outcome::Success(NormalAdminApiKey(claim)),
                 _ => Outcome::Failure((Status::NonAuthoritativeInformation,()))
                }
                
            },
            Err(_) => Outcome::Failure((Status::NonAuthoritativeInformation,())),
            // _ => Outcome::Failure((Status::NonAuthoritativeInformation,())),
        } 
      }
        // return  keys;
     }
}
impl<'a, 'r> FromRequest<'a, 'r> for UserApiKey {
        type Error = ();
        
        fn from_request(request: &'a Request<'r>) -> request::Outcome<UserApiKey, ()>{
          
            let keys: Vec<_> = request.headers().get("authorization").collect();
        
        
            // print!("request header ---> {:#?}",request);
        
            // let nkey = ;
           
             if keys.is_empty() || keys.len() != 1  || keys[0].trim().is_empty() { 
               return  Outcome::Failure((Status::Unauthorized,()));
             }else {
             match verify_token(keys[0]) {
                Ok(claim) => {
                    match claim["role"].as_str() {
                     "1" => Outcome::Success(UserApiKey(claim)),
                     "2" => Outcome::Success(UserApiKey(claim)),
                     "3" => Outcome::Success(UserApiKey(claim)),
                     _ => Outcome::Failure((Status::NonAuthoritativeInformation,()))
                    }
                    
                },
                Err(_) => Outcome::Failure((Status::NonAuthoritativeInformation,())),
                // _ => Outcome::Failure((Status::NonAuthoritativeInformation,())),
            } 
          }
            // return  keys;
         }
        }





