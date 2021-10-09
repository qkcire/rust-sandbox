#![warn(clippy::all, clippy::pedantic)]

fn main() {
    // let MYLIST = [ "One", "Two", "Three"];
    // for i in 0..3 {
    //     println!("{}", MYLIST[i]);
    // }
    let my_list = ["One", "Two", "Three"];
    for num in &my_list {
        println!("{}", num);
    }
}
