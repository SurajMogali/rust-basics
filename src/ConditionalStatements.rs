pub fn condition(){
    let x=3;
    if x<5 {
        println!("conditon is true");
    }
    else{
        println!("conditon is false");  
    }


    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // loop {
    //     println!("again!");
    // }


    let mut number=3;
    while number!=0 {
        println!("Number: {}", number);
        number -= 1;
    }

    let a=[10,20,30,40,50];
    let mut index=0;

    while index<5{
        println!("the value is {}",a[index]);
        index+=1;
    }




}