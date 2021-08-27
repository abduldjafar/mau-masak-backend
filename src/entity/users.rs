// Estructure data for DB
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use bson::DateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,

    pub email: String,
    pub password: String,

    #[serde(default = "default_int_value")]
    pub follower_count: i32,

    #[serde(default = "default_int_value")]
    pub following_count: i32,

    #[serde(default = "time_now")]
    pub created_at: DateTime
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<bson::oid::ObjectId>,
    pub title: String,
    pub description: String,

    #[serde(flatten,skip_serializing_if = "Option::is_none")]
    pub steps : Option<Vec<ReceiptSteps>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<UserComments>>,

    pub like: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiptSteps{
    pub step : i32,
    pub doing: String,
    pub images: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserComments{
    pub from : String,
    pub to: String,
    pub comment: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
fn default_int_value() -> i32 {
    0
}

fn time_now() -> DateTime {
    let local= Utc::now();
    DateTime::from(local)
}
