fn main() {
    let a = Some(2);
    loop {
        match a {
            Some(i) => {
                if i == 2 {
                    break;
                }
            }
            _ => {break;}
        }
    }
    println!("{}", "done");
}
