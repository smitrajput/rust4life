pub mod front_of_house;
pub mod back_of_house;

pub fn eat_at_restaurant() -> String {

  front_of_house::hosting::add_to_waitlist();
    
  back_of_house::cook_order();

  String::from("yummy yummy!")

  // // Absolute path
  // crate::front_of_house::hosting::add_to_waitlist();

  // // Relative path
  // front_of_house::hosting::seat_at_table();
  // hosting::seat_at_table();
  // serving::take_order();
  // serving::serve_order();
  // serving::take_payment();
  // serving::complain();
}

// mod front_of_house {
//   pub mod hosting {
//     pub fn add_to_waitlist() {}
//     pub fn seat_at_table() {}
//   }

//   pub mod serving {
//     pub fn take_order() {}
//     pub fn serve_order() {}
//     pub fn take_payment() {}
//     pub fn complain() {}
//   }
// }

// pub mod back_of_house {
//   fn fix_incorrect_order() {
//     cook_order();

//     // From one level up.
//     super::front_of_house::serving::serve_order();

//     // Absolute path. Crate implies root and lib.rs is the root.
//     crate::front_of_house::serving::take_payment();
//   }

//   fn cook_order() {}
// }