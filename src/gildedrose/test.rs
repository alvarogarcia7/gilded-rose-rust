use super::{Item, GildedRose};

// const mobile_phone: &String = &String::from("Mobile Phone");

#[test]
pub fn foo() {
    let items = vec![Item::new(String::from("foo"), 0, 0)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!("foo", rose.items[0].name);
}

#[test]
pub fn expired_age_brie_increases_quality_twice_as_fast() {
    let items = vec![Item::new(String::from("Aged Brie"), 0, 0)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!(Item::new(String::from("Aged Brie"), -1, 2), rose.items[0]);
}

#[test]
pub fn expired_items_decrease_quality_twice_as_fast() {
    let items = vec![Item::new(String::from("Mobile Phone"), 0, 0)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!(Item::new(String::from("Mobile Phone"), -1, 0), rose.items[0]);
}

#[test]
pub fn expired_items_decrease_quality_when_expired() {
    let items = vec![Item::new(String::from("Aged Brie"), 1, 0)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!(Item::new(String::from("Aged Brie"), 0, 1), rose.items[0]);
}

#[test]
pub fn age_brie_increases_quality_when_expired() {
    let items = vec![Item::new(String::from("Aged Brie"), 1, 0)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!(Item::new(String::from("Aged Brie"), 0, 1), rose.items[0]);
}
