pub struct Config {
    pub ip: String,
    pub port: usize,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let default_port = String::from("7878");
        let port_string = args.get(1).unwrap_or(&default_port).clone();
        let port = match port_string.parse() {
            Ok(t) => t,
            Err(_) => return Err("port is not usize"),
        };
        let default_ip = String::from("0.0.0.0");
        let ip = args.get(2).unwrap_or(&default_ip).clone();
        Ok(Config { ip, port })
    }
}
