use std::collections::HashMap;
use std::time::Instant;

mod filehelper;
mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;

fn main() {
    let day_to_execute = 13;
    let part_to_execute = 2;
    let use_sample = false;

    let mut things_to_execute: HashMap<(i32, i32), fn(bool) -> ()> = HashMap::new();
    things_to_execute.insert((0, 1), day0::exec1);
    things_to_execute.insert((0, 1), day0::exec2);
    things_to_execute.insert((1, 1), day1::exec1);
    things_to_execute.insert((1, 2), day1::exec2);
    things_to_execute.insert((2, 1), day2::exec1);
    things_to_execute.insert((2, 2), day2::exec2);
    things_to_execute.insert((3, 1), day3::exec1);
    things_to_execute.insert((3, 2), day3::exec2);
    things_to_execute.insert((4, 1), day4::exec1);
    things_to_execute.insert((4, 2), day4::exec2);
    things_to_execute.insert((5, 1), day5::exec1);
    things_to_execute.insert((5, 2), day5::exec2);
    things_to_execute.insert((6, 1), day6::exec1);
    things_to_execute.insert((6, 2), day6::exec2);
    things_to_execute.insert((7, 1), day7::exec1);
    things_to_execute.insert((7, 2), day7::exec2);
    things_to_execute.insert((8, 1), day8::exec1);
    things_to_execute.insert((8, 2), day8::exec2);
    things_to_execute.insert((9, 1), day9::exec1);
    things_to_execute.insert((9, 2), day9::exec2);
    things_to_execute.insert((10, 1), day10::exec1);
    things_to_execute.insert((10, 2), day10::exec2);
    things_to_execute.insert((11, 1), day11::exec1);
    things_to_execute.insert((11, 2), day11::exec2);
    things_to_execute.insert((12, 1), day12::exec1);
    things_to_execute.insert((12, 2), day12::exec2);
    things_to_execute.insert((13, 1), day13::exec1);
    things_to_execute.insert((13, 2), day13::exec2);

    /*for day in 0..32 {
        for part in 0..2 {
            println!("day {} part {}:", day, part);
            let thing_to_execute= things_to_execute.get(&(day, part));
            if !thing_to_execute.is_some() {
                println!("unavailable");
                continue;
            }
            let func = thing_to_execute.expect("got it");
            let start = Instant::now();
            func(false);
            let duration = start.elapsed();
            println!("ran in {:?} microseconds, that is {} MS", duration.as_micros(), duration.as_millis());
            println!("");
        }
    }*/

    let thing_to_execute = things_to_execute.get(&(day_to_execute, part_to_execute));
    if thing_to_execute.is_some() {
        let start = Instant::now();
        thing_to_execute.unwrap()(use_sample);
        let duration = start.elapsed();
        println!("ran in {} microseconds, that is {} MS", duration.as_micros(), duration.as_millis());
    }
}
