// use std::collections::HashMap;
// use std::hash::Hash;
use std::thread;
use std::time::Duration;

mod counter;

fn main() {
    println!("Hello, world!");

    let simulated_user_specified_value = 10;
    let simulated_random_value = 7;

    generate_workout(simulated_user_specified_value, simulated_random_value);

    let res = simulated_expensive_calculation(100);

    println!("{}", res);

    counter::counter();
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly");
    //
    //     thread::sleep(Duration::from_secs(2));
    //
    //     intensity
    // };

    // let mut expensive_result = Cacher::new(expensive_closure);

    if intensity < 25 {
        // println!(
        //     "Today, do {} pushups!",
        //     // expensive_result.getValue(random_number)
        // );

        // println!("Next, do {} situps!", expensive_result.getValue(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            // println!(
            //     "Today, run for {} minutes!",
            //     expensive_result.getValue(intensity)
            // );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly");

    thread::sleep(Duration::from_secs(2));

    intensity
}

// <Func, Val, Inp>
// where
//     Inp: PartialEq,
//     Func: Fn(Inp) -> Val,
// {
//     calculation: Func,
//     result_map: HashMap<Inp, Val>,
// }
//
// impl<Func, Val, Inp> Cacher<Func, Val, Inp>
// where
//     Inp: PartialEq + Eq + Hash,
//     Func: Fn(Inp) -> Val,
// {
//     fn new(calculation: Func) -> Cacher<Func, Val, Inp> {
//         Cacher {
//             calculation,
//             result_map: HashMap::new(),
//         }
//     }
//
//     fn getValue(&mut self, arg: Inp) -> Val {
//         for (key, val) in self.result_map.clone() {
//             if key == arg {
//                 return val;
//             }
//         }
//
//         let v = (self.calculation)(arg);
//
//         self.result_map.insert(arg, v);
//
//         v
//     }
// }

// #[cfg(test)]
// mod test {
//     // use crate::Cacher;
//     use super::*;
//
//     #[test]
//     fn call_with_different_valuses() {
//         // let mut cache = Cacher::new(|x| x);
//
//         // let v1 = cache.getValue(1);
//         // let v2 = cache.getValue(2);
//
//         // assert_eq!(v2, 2);
//     }
// }
