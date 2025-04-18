fn main() {
    let s1 = String::from("Hello, This is Ali.");
    let len_s1 = calculate_length (&s1);

    println!("the length of {s1} is {len_s1}");

    let mut s2 = String::from("Hello");
    println! ("before change s2 : {s2}");
    change(&mut s2);
    println!("after change s2 : {s2}");

    let mut s3 = String::from("Mut");
    // let r1 = &mut s3; //! cannot borrow as immutable if once it's mutable.
    // let r2 = &mut s3; //! cannot borrow immutable var more than once.
    let r3 = &s3;
    let r4 = &s3;
    println!("{} {} {}",s3,r3,r4);

    


}

fn calculate_length (s: &String) -> usize {
    s.len()
}

fn change (some_str: &mut String) {
    some_str.push_str(", World")
}
