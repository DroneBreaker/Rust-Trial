use async_std::{fs::File, io, prelude::*, task}

pub fn asyncronous() {
    async fn read_file(path: &str) -> io::Result<String> {
        let mut file = File::open(path).await?;
        let mut contents = String::new(); 
        file.read_to_string(&mut contents).await?;

        Ok(contents)
    }

    let task = task::spawn(async {
        let result = read_file("read.txt").await?

        match result {
            Ok(k) => println!("{}", k)
            Err(e) => println!("Error reading from file: {}", e)
        }
    })
}