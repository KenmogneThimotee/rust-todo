
use crate::service;
pub fn create_status(name: String, color: String, if_exist: String){

    service::create_status(name, color, if_exist);
}