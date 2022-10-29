use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }

    let rem = (a * b) % 2;
    println!("{}", if rem == 0 { "Even" } else {"Odd"});
}
