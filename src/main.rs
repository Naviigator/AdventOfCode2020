use std::collections::HashMap;

mod filehelper;
mod day0;
mod day1;

fn main() {
    let day_to_execute = 1;
    let part_to_execute = 2;
    let use_sample = false;

    let mut things_to_execute: HashMap<(i32, i32), fn(bool) -> ()> = HashMap::new();
    things_to_execute.insert((0, 1), day0::exec1);
    things_to_execute.insert((0, 1), day0::exec2);
    things_to_execute.insert((1, 1), day1::exec1);
    things_to_execute.insert((1, 2), day1::exec2);

    let thing_to_execute = things_to_execute.get(&(day_to_execute, part_to_execute));
    if thing_to_execute.is_some() {
        thing_to_execute.unwrap()(use_sample);
    }
}
