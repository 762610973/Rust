#![allow(unused)]

// use crate::front_of_house::hosting; // 使用绝对路径
use back_of_house as bfh;
use front_of_house::hosting; // 相对路径
pub use front_of_house::serving;

mod front_of_house;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_wait_list();
    // relative path
    front_of_house::hosting::add_to_wait_list();
    let mut meal = back_of_house::BreakFast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    hosting::add_to_wait_list();
    test::add_to_wait_list();
    //-----------------------------
    bfh::fix_incorrect();
}

fn serve_order() {}

mod back_of_house {
    pub fn fix_incorrect() {
        cook_order();
        super::serve_order();
        super::front_of_house::hosting::add_to_wait_list();
        super::eat_at_restaurant();
        super::back_of_house::cook_order();
        // super::back_of_house::fix_incorrect();
        // super::front_of_house::StructMod;
    }

    fn cook_order() {}

    pub struct BreakFast {
        pub toast: String,
        seasonal_fruit: String, // 这个逗号文件保存时会自动格式化添加上去,但是写的时候要注意,不要忘记
    }

    impl BreakFast {
        pub fn summer(toast: &str) -> BreakFast {
            BreakFast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
