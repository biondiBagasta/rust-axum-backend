use reqwest::{ Client };
use once_cell::sync::Lazy;

pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub const JWT_SECRET: &str = "a3budrspk2m";