//name:gaopenghao

fn main() {
    let mut s = String::from("hello");
    let a = &s;
    println!("{}", s);
    println!("{}", a);
    println!("{:p}", a);

    change(&mut s);//可变引用
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}


