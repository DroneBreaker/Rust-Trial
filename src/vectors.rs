pub fn vector() {
    let mut num: Vec<i32> = vec![];

    num.push(1);
    num.push(2);
    num.push(3);

    let pop = num.pop();

    println!("{:?}", pop);
    println!("{}", num.len());
    println!("{}", num.is_empty())
}