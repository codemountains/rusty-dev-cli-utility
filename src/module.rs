use crate::module::sub_module::how_are_you_doing;

mod sub_module;

pub fn nice_to_meet_you() {
    println!("Nice to meet you.");
    how_are_you_doing();
}
