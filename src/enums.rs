pub enum Pet {
    Dog,
    Cat,
    Fish
}

impl Pet {
    fn what_am_I(self) -> &'static str {
        match self {
            Pet::Dog => "I am a dog",
            Pet::Cat => "I am a cat",
            Pet::Fish => "I am a fish",
        }
    }

    // println!("{}", dog.what_am_I());
}

pub enum IpAddrKind {
    V4,
    V6
}

pub struct IpAddr {
    kind: IpAddrKind,
    address: String
}
