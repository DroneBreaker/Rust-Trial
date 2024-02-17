pub fn exercises() {
    // let var1 = 5;
    // let var2 = 2;

    // let var3 = var1 % var2;

    // println!("{}", var3)

    // let mut vec = vec!(2, 4, 6, 8, 10);
    // println!("{:?}", vec);

    // vec.pop();
    // vec.push(12);
    // println!("{:?}", vec);


    // if a > 50 {
    //     println!("The value is greater than 50")
    // } else if a == 1 {
    //     println!("The value is one")
    // } else if a > 25 || a < 50 {
    //     println!("The value is greater than 25 but less than 50")
    // }

    //  
    
    // struct Car {
    //     mpg: u32,
    //     color: String,
    //     top_speed: u32
    // }

    // impl Car {
    //     // Method to set the MPG
    //     fn set_mpg(&mut self, new_mpg: u32) {
    //         self.mpg = new_mpg;
    //     }
    
    //     // Method to set the color
    //     fn set_color(&mut self, new_color: String) {
    //         self.color = new_color;
    //     }
    
    //     // Method to set the top speed
    //     fn set_top_speed(&mut self, new_top_speed: u32) {
    //         self.top_speed = new_top_speed;
    //     }
    // }

    // let mut my_car = Car {
    //     mpg: 25,
    //     color: String::from("Red"),
    //     top_speed: 120,
    // };

    // // Set new values for mpg, color, and top speed
    // my_car.set_mpg(30);
    // my_car.set_color(String::from("Blue"));
    // my_car.set_top_speed(150);

    // // Print the values of the car
    // println!("Car details:");
    // println!("MPG: {}", my_car.mpg);
    // println!("Color: {}", my_car.color);
    // println!("Top Speed: {} mph", my_car.top_speed)

    // pub enum Shape {
    //     Triangle,
    //     Square,
    //     Pentagon,
    //     Octagon
    // }

    // impl Shape {
    //     fn corners(&self) -> u8 {
    //         match self {
    //             Shape::Triangle => 3,
    //             Shape::Square => 4,
    //             Shape::Pentagon => 5,
    //             Shape::Octagon => 8,
    //         }
    //     }
    // }

    // let triangle = Shape::Triangle;
    // let square = Shape::Square;

    // println!("Number of corners in a triangle: {}", triangle.corners());
    // println!("Number of corners in a square: {}", square.corners());

    let vec = vec![1, 3, 5, 7, 9];
    let mut newVec = Vec::new();

    vec.iter().for_each(|&x| {
        newVec.push(x * 10);
    });

    println!("{:?}", newVec);
}