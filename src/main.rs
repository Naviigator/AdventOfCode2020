use std::collections::HashMap;

mod filehelper;
mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let day_to_execute = 6;
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

    let thing_to_execute = things_to_execute.get(&(day_to_execute, part_to_execute));
    if thing_to_execute.is_some() {
        thing_to_execute.unwrap()(use_sample);
    }
}
