use crate::model::Status;

pub fn create_status(name: String, color: String, if_exist: String){

    let status = Status{
        name: name,
        color: color
    };
    println!("Status {:?}", status);
}