mod front_of_house;

// FILL in the blank and FIX the errors
fn main() {
    assert_eq!(rust_ballistics::front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(rust_ballistics::eat_at_restaurant(), "yummy yummy!");

    println!("{}", rust_ballistics::eat_at_restaurant());
    println!("{}", rust_ballistics::front_of_house::hosting::seat_at_table());
}
