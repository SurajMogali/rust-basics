pub fn stringgenerate(){
    let s1=String::from("Hello");
    let len=calculate_length(&s1);
    println!("the length of '{s1}' is {len}");


    let mut s = String::from("hello");

    change(&mut s);

    print!("{}",s);




}


fn calculate_length(s:&String)->usize{
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}