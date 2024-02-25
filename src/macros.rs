pub fn mac() {
    macro_rules! drone {
        ($a: expr, $b: expr) => {
            {
                let mut m = $b;
                let mut n = $a;

                while m != 0 {
                    if m < n {
                        let t = m;
                        m = n;
                        n = t;
                    }

                    m = m % n;
                }

                n
            }
        };
    }

    println!("{:?}", drone!(14, 15));
}