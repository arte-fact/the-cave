use crate::game::item::Item;

pub struct Name(String);
pub struct Attack(i32);
pub struct Defense(i32);
pub struct Health(i32);
pub struct Loot(Vec<(Item, i32)>);
pub struct Size(i32);


