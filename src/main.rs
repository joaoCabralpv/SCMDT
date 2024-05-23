mod input;
mod menu;
use menu::Tool;
fn main() {
    loop {
        menu::menu(&[
            Tool::from(f1,String::from("f1")),
            Tool::from(f2,String::from("f2")),
            Tool::from(f3,String::from("f3")),
            ]);
    }
}

fn f1(){println!("f1")}
fn f2(){println!("f2")}
fn f3(){println!("f3")}