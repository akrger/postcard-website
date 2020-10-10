use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize)]
pub struct Card {
    pub id: i32,
}
