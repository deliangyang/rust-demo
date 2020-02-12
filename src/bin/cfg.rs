
#[derive(Debug)]
struct Test {
    name: String,
}

impl Clone for Test {
    fn clone(&self) -> Self {
        Test {
            name: String::from(&self.name),
        }
    }
}

fn main() {
    let a = Test {
        name: String::from("xxxxxx"),
    };
    let mut b = a.clone();
    b.name = String::from("123123");
    println!("{:?}, {:?}", a, b)
}
