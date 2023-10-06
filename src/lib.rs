#![allow(dead_code,unused_variables)]


mod database{
  pub  enum Status {
        Connected,
        Interrupted
    
    }
  pub  fn connect_to_database() -> Status{
        return Status::Connected
 }

    pub fn get_user(){

} 

}

mod auth_utlis{



pub fn login(creds:models::Credentials) {
    crate::database::get_user();
}

fn logout(){

}
pub  mod models{
    pub struct Credentials{
        username: String,
        password: String
    }

}

} 



fn get_user(){

}
use auth_utlis::models::Credentials;
use database::Status;

pub fn authenticate(creds:Credentials){
    if let  Status::Connected = database::connect_to_database() {
       auth_utlis:: login(creds);
    }


}