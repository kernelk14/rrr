// use std::vec::Vec;
fn eefie() {
    let mut i: i64;
    for mut i in 1..10 {
        println!("{}", i);
        i = i + 1;
    }
}

fn main() {
    let mut st = Vec::<&str>::new();

    st.push("This is a text.");
    let a = st.pop();

    println!("{:?}", a);
    if true {
        return eefie();
    }
}
