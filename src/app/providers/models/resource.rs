use serde::{Deserialize, Serialize};

use crate::app::providers::models::question::PubQuestion;
use crate::app::providers::models::slide::PubSlide;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubResourceContent {
    pub slides: Option<Vec<PubSlide>>,
    pub form: Option<Vec<PubQuestion>>,
    pub external: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubResource {
    pub id: i32,
<<<<<<< HEAD
=======
    pub resource_type: String,
>>>>>>> 0b9aa827104d7b7a68bdca13745b0562a965efc0
    pub title: String,
    pub description: String,
    pub content: Option<PubResourceContent>,
}
