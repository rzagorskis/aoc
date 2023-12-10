/*

--- Part Two ---

Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it looks like the seeds: line actually describes ranges of seed numbers.

The values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range. So, in the first line of the example above:

seeds: 79 14 55 13

This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.

Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.

In the above example, the lowest location number can be obtained from seed number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and location 46. So, the lowest location number is 46.

Consider all of the initial seed numbers listed in the ranges on the first line of the almanac. What is the lowest location number that corresponds to any of the initial seed numbers?


*/

use crate::{io_utils::read_lines_fully, day5::defs::SeedRangePair};

use super::{almanac::parse_almanac, defs::Almanac};

const EXPECTED_ANSWER: i64 = 31599214;

fn binary_search(almanac: &Almanac, seed_range_pair: &SeedRangePair, lowest_location: &mut i64) {
    let mut low = seed_range_pair.start;
    let mut high = seed_range_pair.end;

    while low <= high {
        let mid = low + (high - low) / 2;

        let soil = almanac.seed_to_soil_map.destination(mid);
        let fertilizer = almanac.soil_to_fertilizier_map.destination(soil);
        let water = almanac.fertizilier_to_water_map.destination(fertilizer);
        let light = almanac.water_to_light_map.destination(water);
        let temperature = almanac.light_to_temperature.destination(light);
        let humidity = almanac.temperature_to_humidity.destination(temperature);
        let location = almanac.humidity_to_location.destination(humidity);

        if location < *lowest_location {
            *lowest_location = location;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
}

pub fn run() {
    let lines = read_lines_fully("src/day5/input.txt");

    if let Some(almanac) = parse_almanac(lines) {
        let mut lowest_location: i64 = i64::MAX;

        assert_eq!(almanac.seeds.len() % 2, 0);

        let mut seed_range_pairs = Vec::<SeedRangePair>::new();

        let iterations = almanac.seeds.len() / 2;

        let mut first_ix = 0;
        let mut second_ix = 1;

        for _ in 0..iterations {
            let seed_range_start = *almanac.seeds.get(first_ix).unwrap();
            let seed_range_length = *almanac.seeds.get(second_ix).unwrap();

            seed_range_pairs.push(SeedRangePair {
                start: seed_range_start,
                end: seed_range_start + seed_range_length - 1,
                length: seed_range_length,
            });

            first_ix += 2;
            second_ix += 2;
        }

        for seed_range in &seed_range_pairs {
            for seed in seed_range.start..=seed_range.end {
                let mut location = seed;
                location = almanac.seed_to_soil_map.destination(location);
                location = almanac.soil_to_fertilizier_map.destination(location);
                location = almanac.fertizilier_to_water_map.destination(location);
                location = almanac.water_to_light_map.destination(location);
                location = almanac.light_to_temperature.destination(location);
                location = almanac.temperature_to_humidity.destination(location);
                location = almanac.humidity_to_location.destination(location);
    
                if location < lowest_location {
                    lowest_location = location;
                }
            }
        }

        // TODO: the trick seems to be to use reverse lookups since location is the important value, not the seed.

        for seed_range_pair in seed_range_pairs {
            binary_search(&almanac, &seed_range_pair, &mut lowest_location);
        }

        // assert_eq!(EXPECTED_ANSWER, lowest_location);

        println!("{}", lowest_location);
    }
}
