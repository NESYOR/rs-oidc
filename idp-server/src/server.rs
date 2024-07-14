use std::collections::HashMap;
use types::{Config, LoginRequest};
use std::sync::{Mutex,Arc};

#[derive(Clone)]
pub struct Server {
    private_key:Vec<u8>,
    config: Config,
    loginrequest: Arc<Mutex<HashMap<String,LoginRequest>>>,
    codes: Arc<Mutex<HashMap<String, LoginRequest>>>,
}

impl Server{
    fn new(private_key:Vec<u8>,config:Config)-> Self{
        Server{
            private_key,
            config,
            loginrequest: Arc::new(Mutex::new(HashMap::new())),
            codes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

     // Public method to access the private_key
     pub fn get_private_key(&self) -> &[u8] {
        &self.private_key
    }
}
