/*
--- Part Two ---

As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has very bad kerning. There's really only one race - ignore the spaces between the numbers on each line.

So, the example from before:

Time:      7  15   30
Distance:  9  40  200

...now instead means this:

Time:      71530
Distance:  940200

Now, you have to figure out how many ways there are to win this single race. In this example, the race lasts for 71530 milliseconds and the record distance you need to beat is 940200 millimeters. You could hold the button anywhere from 14 to 71516 milliseconds and beat the record, a total of 71503 ways!

How many ways can you beat the record in this one much longer race?


*/

use crate::io_utils::{extract_numbers_from_line, read_lines_fully};

#[derive(Debug)]
struct RaceData {
    time: u64,
    record_distance: u64,
}

const EXPECTED_ANSWER: u64 = 27340847;

fn parse_race_data() -> RaceData {
    let lines = read_lines_fully("src/day6/input.txt");

    assert_eq!(lines.len(), 2);

    let line_times = lines.first().unwrap();
    let line_distances = lines.last().unwrap();

    let times = extract_numbers_from_line::<u64>(line_times);
    let distances = extract_numbers_from_line::<u64>(line_distances);

    let the_time = times.into_iter().map(|digit| digit.to_string()).collect::<String>().parse().unwrap();
    let the_distance = distances.into_iter().map(|digit| digit.to_string()).collect::<String>().parse().unwrap();

    return RaceData {
        time: the_time,
        record_distance: the_distance
    };
}

pub fn run() {
    let race_data_item = parse_race_data();

    println!("{:?}", race_data_item);

    let mut winning_race_variations = 1_u64;

    let mut button_hold_time_from_start = 1;

    loop {
        let distance_start = button_hold_time_from_start * (race_data_item.time - button_hold_time_from_start);

        if distance_start > race_data_item.record_distance {
            break;
        }

        button_hold_time_from_start += 1;
    }

    let mut button_hold_time_from_end = race_data_item.time;

    loop {
        let distance_end = button_hold_time_from_end * (race_data_item.time - button_hold_time_from_end);

        if distance_end > race_data_item.record_distance {
            break;
        }

        button_hold_time_from_end -= 1;
    }

    let winning_variations = (button_hold_time_from_end - button_hold_time_from_start + 1) as u64;

    winning_race_variations *= winning_variations;

    println!("earliest - {}, latest - {}", button_hold_time_from_start, button_hold_time_from_end);
    println!("Variations: {}", winning_variations);

    assert_eq!(EXPECTED_ANSWER, winning_race_variations);

    println!("Total winning variations: {}", winning_race_variations);
}
