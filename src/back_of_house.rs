pub fn fix_incorrect_order() {
  cook_order();

  // From one level up.
  super::front_of_house::serving::serve_order();

  // Absolute path. Crate implies root and lib.rs is the root.
  crate::front_of_house::serving::take_payment();
}

pub  fn cook_order() {}
