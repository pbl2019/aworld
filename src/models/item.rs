use crate::define_enum;
use crate::schema::items;
use crate::utils::generate_random_name;
use num_traits::{FromPrimitive, ToPrimitive};
use rand::{thread_rng, Rng};

define_enum! {
    #[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy)]
    pub enum ItemType {
        Unknown = 0,
        Food = 1,
        Weapon = 2,
    }
}

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub item_type: ItemType,
    pub amount: i64,
    pub is_used: bool,
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem {
    pub name: String,
    pub item_type: ItemType,
    pub amount: i64,
    pub is_used: bool,
}

impl std::default::Default for NewItem {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            name: generate_random_name(0),
            item_type: ItemType::from_u64(rng.gen_range(0, 2 + 1)).unwrap(),
            amount: rng.gen_range(0, 1000),
            is_used: false,
        }
    }
}

#[test]
fn create_item() {
    let new_item = NewItem::default();
    assert!(new_item.name.len() > 0);
    // item_type must be lower than number of all kinds of item type
    assert!(new_item.item_type.to_u32().unwrap() <= 2);
    assert!(new_item.amount >= 0);
}
