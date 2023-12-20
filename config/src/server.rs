use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub port: u16,
}

impl  ServerConfig{
    pub fn get_server_addrs(&self)->String{
        format!("{}:{}",self.addr,self.port)

    }
    pub  fn get_server_http_addrs(&self)->String{
        format!("http://{}:{}", self.addr, self.port)
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


#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    pub fn app_config_http_addr_test() {
        let config = ServerConfig {
            addr: "127.0.0.1".to_string(),
            port: 1024,
        };
        assert_eq!(config.get_server_http_addrs(), "http://127.0.0.1:1024");
    }
    #[test]
    pub fn app_config_http_addrs_is_empty_test(){
        let config=ServerConfig{
            addr: "".to_string(),
            port: 0,
        };
        assert_eq!(config.check_if_server_is_empty(), true);
    }
}


