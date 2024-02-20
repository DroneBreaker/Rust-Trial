pub fn decision_making() {
    // let one = 1;

    // if one > 0 {
    //     println!("true")
    // } else if one == 1 {
    //     println!("equal")
    // } else {
    //     println!("false")
    // }

    // let mut num = 0;

    // 'counter: loop {
    //     println!("Count: {}", num);
    //     let mut decrease = 5;

    //     loop {
    //        println!("Decreasing: {}", decrease);

    //        if decrease == 4 {
    //         break;
    //        }

    //        if num == 2 {
    //         break 'counter;
    //        }

    //        decrease -= 1;
    //     }

    //     num += 1;
    // }

    // WHILE LOOP
    // let mut num = 0;

    // while num < 5 {
    //     println!("{}", num);
    //     num += 1;
    // }

    // FOR LOOP
    let vec: Vec<i8> = (0..10).collect();

    for element in vec {
        println!("{}", element);
    }
}
