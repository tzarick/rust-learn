fn main() {
    let mut s = String::from("ahoy m8y!");
    {
        let r1 = &mut s;
    }

    let r2 = &mut s;
    println!("{}", r2);
    do_something(&s);

    use_mut_ref(&mut s);

    println!("{}", s);
}

fn do_something(str: &String) {
    println!("{}", str);
    // str.push_str("something"); // -- doesn't work because the reference is immutable
}

fn use_mut_ref(str: &mut String) {
    str.push_str("something"); // works because the reference is mutable
}
