use std::io;

// fn main(){
//     println!("Fahrenheit to Celsius Converter");
//     loop { //Allow users to switch continuously rather than exit after a single run.
//         println!("Please enter temperature in Fahrenheit (press q to quit): ");

//         let mut input = String::new();

//         io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");

//     let input = input.trim(); //Remove the newline character \n from user input.

//     if input == "q" {
//         break;
//     }

//     let fahrenheit: f64 = match input.parse(){ //Core method for converting strings to numbers.
//         Ok(num) => num,
//         Err(_) => {
//             println!("Please enter a valid number or 'q' to quit.");
//             continue;
//         }
//     }; //Handle errors gracefully (if the user enters a non-numeric value, prompt them to try again instead of crashing the program).

//     let celsius = (fahrenheit - 32.0) / 1.8;
//     println!("{:.2}°F is {:.2}°C", fahrenheit, celsius);
//     }
// }


// fn main(){
//     println!("Fibonacci Sequence Generator");
//     println!("Please enter n: ");

//     let mut n = String::new();
//     io::stdin().read_line(&mut n).expect("Error");

//     let n: u32 = n.trim().parse().expect("Please enter a integer number");

//     let result = fib(n);
//     println!("Fibonacci number at position {} is {}", n, result);
// }

// fn fib(n: u32) -> u64{
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     }

//     let mut a = 0;
//     let mut b = 1;
//     let mut sum = 0;

//     for _ in 2..=n{
//         sum = a + b;
//         a = b;
//         b = sum;
//     }
//     sum
// }

fn main(){
    let days = [
        "first", "second", "third", "fourth", "firth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "a Partridge in a Pear Tree.",
        "two Turtle Doves,",
        "three French Hens,",
        "four Calling Birds,",
        "five Gold Rings,",
        "six Geese a Laying,",
        "seven Swans a Swimming,",
        "eight Maids a Milking,",
        "nine Ladies Dancing,",
        "ten Lords a Leaping,",
        "eleven Pipers Piping,",
        "twelve Drummers Drumming,"
    ];

    for i in 0..12{
        println!("On the {} day of Christmas, my true love sent to me", days[i]);
        for j in (0..=i).rev(){
            if i != 0 && j == 0{
                print!("and ");
            }
            println!("{}", gifts[j]);
        }
        println!();        
    }
}