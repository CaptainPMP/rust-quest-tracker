use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::{domain::entities::{adventurers::AdventurerEntity, quests::QuestEntity}, infrastructure::postgres::schema::quest_adventurer_junction};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Associations)]
#[diesel(belongs_to(AdventurerEntity, foreign_key = adventurer_id))]
#[diesel(belongs_to(QuestEntity, foreign_key = quest_id))]
#[diesel(table_name = quest_adventurer_junction)]
pub struct QuestAdventurerJunction {
    pub quest_id: i32,
    pub adventurer_id: i32,
}