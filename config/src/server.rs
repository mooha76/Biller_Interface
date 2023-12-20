use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub port: u16,git
}

impl  ServerConfig{
    pub fn get_server_addrs(&self)->String{
        format!("{}:{}",self.addr,self.port)

    }
    pub  fn get_server_http_addrs(&self)->String{
        format!("http:/{}:{}", self.addr, self.port)
    }
    pub fn check_if_server_is_empty(&self)->bool{
        self::is_server_adds_port_empty(
            &self.addr,
            self.port
        )
    }
}

pub fn is_server_adds_port_empty(addrs:&str , port:u16)->bool{
    if addrs.trim().is_empty() || port==0{
        return  false
    }
    return  true
}