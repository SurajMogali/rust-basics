use std::collections::HashMap;
use std::iter;

use std::path;

use std::fs;
use std::sync::mpsc;
use std::thread::spawn;
use std::time::Duration;

use sql::{grammar::statement, Error};
use Enums::{IpAddr, IpAddrKind};
use FirstWordFinder::first_word;
use MatchOperation::Coin;
use Trait::Summary;

use std::thread;

mod functionCall;

mod StatementsAndExpressions;

mod Vector;

mod ConditionalStatements;

mod ReferencesAndBorrowing;

mod FirstWordFinder;

mod Enums;

mod MatchOperation;

mod setuser;

mod Trait;

mod Generics;

mod AsyncFunction;

enum Result<A, B> {
    Ok(A),
    Err(B),
}

fn main() {
    // let mut x:i8=10;
    // for i in 0..1000{
    //     x=x+100;

    // }

    // println!("x={}",x);

    let t: bool = true;
    let f: bool = false;

    println!("{}", t);
    println!("{}", f);

    //String
    let greeting: String = String::from("hello suraj");
    println!("{}", greeting);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //Char
    let c = 'z';
    let z: char = 'Z';
    println!("{}", c);

    //Tuple
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    //Array

    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    //To assign all the elements same number only
    //let a = [3; 5];

    let first = a[0];
    let second = a[1];

    println!("{}", first);
    println!("{}", second);

    functionCall::anotherfunction(78);
    StatementsAndExpressions::statement();

    let x = StatementsAndExpressions::five();
    println!("x={}", x);

    ConditionalStatements::condition();

    ReferencesAndBorrowing::stringgenerate();

    let mut s = String::from("Honnur Ali");
    let word = first_word(&s);
    println!("{}", word);

    //String slice
    let str1 = String::from("hello world");
    let hello = &str1[0..5];
    println!("{}", hello);

    let str2 = String::from("hello");

    //Both are same, u can drop the 0 here
    let slice = &str2[0..2];
    let slice = &str2[..2];

    println!("{}", slice);

    //Same like above,we can drop the last
    let str3 = String::from("hello");
    let len = str3.len();
    let slice = &str3[3..len];
    let slice = &str3[3..];
    println!("{}", slice);

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    //Just like the javascript,we can copy the properties of one object to the other
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user2.username);

    //Structs with tuple-like syntax
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    struct Rectangle {
        width: u32,
        height: u32,
    }

    //This is similar to extending the class
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    };

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", rect.area());

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let cost = MatchOperation::value_in_cents(Coin::Penny);

    print!("{}", cost);

    // let res=read_from_file_unsafe("example.txt".to_string());

    // match res{
    //     Ok(content)=>{
    //         println!("File content: {}",content);
    //     },
    //     Err(err)=>{
    //         println!("Error:{}",err);
    //     }

    // }
    print!("hi there");

    let p = Generics::Point { x: 5, y: 10 };

    println!("Point coordinates: ({}, {})", p.x, p.y);

    //Vectors

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    println!("{:?}", vec);

    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);
    numbers.push(6);
    numbers.push(7);
    numbers.push(8);

    let mut evenNumbers = Vector::even_filter(numbers);

    println!("{:?}", evenNumbers);

    //HashMap

    let mut users = HashMap::new();
    users.insert(String::from("harkirat"), 22);
    users.insert(String::from("raman"), 32);

    let first_user_age = users.get("harkirat");

    println!("{:?}", first_user_age);

    match first_user_age {
        Some(age) => println!("age is {}", age),
        None => println!("User not found in the db"),
    }

    let input_vec = vec![(String::from("harkirat"), 22), (String::from("raman"), 32)];
    let hm = group_values_by_keys(input_vec);

    println!("{:?}", hm);

    //Iterator
    let v1 = vec![1, 2, 3];
    for val in v1.iter() {
        println!("Got:{}", val);
    }

    let mut nums = vec![1, 2, 3];
    let iter = nums.iter_mut();

    //Iter.next()
    let mut nums_iter = nums.iter_mut();

    while let Some(val) = nums_iter.next() {
        println!("{}", val);
    }

    //Iterator- Adapter(map,filter,)
    let iter3 = v1.iter();

    let iter2 = iter3.map(|x| x + 1);
    for i in iter2 {
        println!("{}", i);
    }

    //---------------------------------------------------------------------------------------------

    let iter5 = v1.iter();
    let iter4 = iter5.filter(|x| *x % 2 == 0);
    for i in iter4 {
        println!("{ }", i);
    }

    let user = Trait::User {
        name: String::from("Harkirat"),
        age: 21,
    };

    println!("{:?}", user.summarize());

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from the spawned thread! {}", i);
        }
    });

    // The main thread will continue its own work concurrently.
    for i in 1..5 {
        println!("Hello from the main thread! {}", i);
    }

    // Wait for the spawned thread to finish.
    handle.join().unwrap();

    //MesaagePassing
    let (tx, rx) = mpsc::channel();
    spawn(move || tx.send(String::from("hello world!")));

    let value = rx.recv();
    match value {
        Ok(val) => println!("{}", val),
        Err(err) => println!("Error: {}", err),
    }

    // let data = AsyncFunction::fetch_data().await;
    // println!("Fetched data: {}", data);

    // for val in iter{
    //     *val=*val+1;
    // }

    // println!("{:?}",nums);

    for i in 1..6 {
        println!("{}", i);
    }

    let mut numbers12: [i32; 5] = [1, 2, 3, 4, 5];
    numbers12[2] = 0;

    let mut sum12 = 0;

    for p in 0..numbers12.len() {
        println!("{}", numbers12[p]);
        sum12 += numbers12[p];
        println!("{}", sum12);
    }

    let a = [19, 46, 33, 64, 95, 11, 5];
    let mut max;
    let mut min;
    let n = a.len();

    max = a[0];
    for i in 1..n {
        if a[i] > max {
            max = a[i];
        }
    }

    println!("Max number is {}", max);

    min = a[0];
    for i in 1..n {
        if a[i] < max {
            min = a[i];
        }
    }

    println!("Min number is {}", min);

    let (sum, diff) = sumdiff(4, 1);
    println!("Sum is {} and diff is {}", sum, diff);

    let person1 = Person {
        name: String::from("Alice"),
        age: 25,
        is_student: true,
    };

    let person2 = Person {
        name: String::from("Alice"),
        age: 25,
        is_student: true,
    };



    //Destructin the structure
    let Person{name,age,is_student}=person2;


    println!("Name:{}", name);     
    println!("Age:{}", age);
    println!("IsStudent:{}", is_student);




    println!("Name:{}", person1.name);
    println!("Age:{}", person1.age);
    println!("IsStudent:{}", person1.is_student);


    let mut s=String::from("My name is Suraj");
    println!("Original string is:{}",s);

    s.push_str(" programming");

    println!("Updated string:{}",s);

    




}

//  fn read_from_file_unsafe(String) -> Result<String,String> {
//     let res=fs::read_to_string("example.txt");

//     if let Ok(content)=res{
//         return Ok(content);
//     }
//     else{
//         return Err("Error reading file".to_string());
//     }

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}

fn sumdiff(a: i32, b: i32) -> (i32, i32) {
    return (a + b, a - b);
}

struct Person {
    name: String,
    age: u32,
    is_student: bool,
}
