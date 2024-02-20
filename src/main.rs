// use std::vec;

// use exercise::exercises;

use crate::traits::Overview;

mod arr;
mod closures;
mod concurrency;
mod control_flow;
mod data_types;
mod enums;
mod exercise;
mod string_literals;
mod strings_and_str;
mod test;
mod traits;
mod variables;
mod vectors;

use vectors::vector;

// use std::io;

fn main() {
    // println!("Enter your weight (kg)");

    // let mut input =  String::new();

    // io::stdin().read_line( &mut input).unwrap();

    // let weight: f32 = input.trim().parse().unwrap();
    // println!("Input: {}", input);

    // let _mars_weight = weight_calculations(100.0);
    // mars_weight = mars_weight * 1000.0;

    // println!("Weight on mars : {}kg!", weight_calculations(100.0));

    // data_types::data_types();
    // variables::variables();
    // strings_and_str::output_str();
    // string_literals::string_literals();
    // control_flow::decision_making();

    // let mut my_vector = vec![1, 3, 5, 7];
    // exercise::exercises();
    // let mut my_car = exercises.();
    // print!("{:?}", exercise::exercises(&mut my_vector));
    // my_vector.push(15);
    // enums::Pet::Dog;
    // enums::IpAddrKind::V4;

    // let dog = enums::Pet::Dog;
    // println!("{}", );
    // traits::Course;
    // let course1 = traits::Course{headline: String::from("headline"), author: String::from("Browne")};
    // let course2 = traits::AnotherCourse {headline: String::from("Another headline"), author: String::from(" Another Browne")};

    // println!("{:?}", course1.overview());
    // println!("{:?}", course2.overview());

    // crate::vector();
    // test::tests::it_works();
    // test::tests::it_fails();
    // let a = crate::closures::City{city: String::from("A"), population: 100};

    // println!("{:?}", a);
    concurrency::concurrency();
}

fn weight_calculations(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
