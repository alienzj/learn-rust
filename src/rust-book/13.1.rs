use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    //let add_one_v3 = |x: u32|        { x + 1 };
    //let add_one_v4 = |x|               x + 1  ;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // type locked in the closure
    // let n = example_closure(5);
}

/*
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
*/

fn generate_workout(intensity: u32, random_number: u32) {
    //let expensive_result = simulated_expensive_calculation(intensity);
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

/*
calculating slowly...
Today, do 10 pushups!
Next, do 10 situps!

calculating slowly...
Today, do 10 pushups!
calculating slowly...
Next, do 10 situps!
*/
