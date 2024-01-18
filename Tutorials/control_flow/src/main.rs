fn main() {
    let number = 7777777;

    if number < 5 {
        println!("condition was true"); 
    } else {
        println!("condition was false"); 
    }

    // using if statements with let statements 
    let condition = true; 
    let number = if condition { 5 } else { 6 }; 

    println!("The value of number is: {number}");

    // repeating code with loop 
    // returning values from loops 
     
    let mut counter = 0; 

    let result = loop {
    counter += 1; 
            
        if counter == 10 {
            break counter * 2; 
        } 
    };

    println!("The result is {result}"); 

    // while loops 
    let a = [10, 20, 30, 40, 50];
    let mut index = 0; 

    while index < 5 {
        println!("the value is: {}", a[index]); 

        index += 1; 
    } 


    // for loop
    // for element in a {
    //     println!("the value is: {element}"); 
    // }

    // for loop that reverses the way numbers are counted 
    // for number in (1..4).rev(); 
}
