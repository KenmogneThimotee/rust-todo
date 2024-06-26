pub(crate) mod status;
pub(crate) mod task;
use clap::builder::Str;

use crate::service::Service;

pub struct Controller {
    service: Service
}

impl Controller {
    
    pub fn new(database_url: String) -> Self {

        let service = Service::new(database_url);

        Controller{
            service: service
        }
    }
}