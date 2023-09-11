use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Message{
    pub account: i32,
    // API = 1, SMS = 2, MAIL = 3, WHATSAPP = 4
    pub destination_type: i32,
    pub phone_number: String,
    pub mail_address: String,
    pub content: String,
}