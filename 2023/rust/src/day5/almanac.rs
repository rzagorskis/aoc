use std::collections::HashMap;

use super::defs::{Almanac, MapEntryDescription, MapType, SourceToDestinationMap};

pub fn parse_almanac(lines: Vec<String>) -> Option<Almanac> {
    let mut lines_iter = lines.iter();

    let seeds_line_split: Vec<&str> = lines_iter.next().unwrap().split(": ").collect();
    let seeds_line: &str = seeds_line_split.get(1).unwrap();

    let seeds_str: Vec<&str> = seeds_line.split(" ").collect();
    let seeds: Vec<i64> = seeds_str
        .iter()
        .filter(|str| !str.is_empty())
        .map(|num_str| num_str.parse().unwrap())
        .collect();

    let mut maps = HashMap::<MapType, Vec<MapEntryDescription>>::new();

    let mut map_read_type = Option::<MapType>::None;
    let mut map_read = Vec::<MapEntryDescription>::new();

    for (ix, line) in lines_iter.enumerate() {
        if map_read_type.is_none() {
            if line.starts_with("seed-to-soil map") {
                map_read_type = Some(MapType::SeedToSoil);
                continue;
            } else if line.starts_with("soil-to-fertilizer") {
                map_read_type = Some(MapType::SoilToFertilizer);
                continue;
            } else if line.starts_with("fertilizer-to-water") {
                map_read_type = Some(MapType::FertizilerToWater);
                continue;
            } else if line.starts_with("water-to-light") {
                map_read_type = Some(MapType::WaterToLight);
                continue;
            } else if line.starts_with("light-to-temperature") {
                map_read_type = Some(MapType::LightToTemperate);
                continue;
            } else if line.starts_with("temperature-to-humidity") {
                map_read_type = Some(MapType::TemperatureToHumidity);
                continue;
            } else if line.starts_with("humidity-to-location") {
                map_read_type = Some(MapType::HumidityToLocation);
                continue;
            }
        }

        // -2 since we .next() at the start to get the seeds
        if line.is_empty() || ix == lines.len() - 2 {
            if let Some(map_read_type) = map_read_type {
                maps.insert(map_read_type, map_read.clone());
            }

            map_read_type = None;
            map_read.clear();

            continue;
        }

        let map_entry_split: Vec<&str> = line.split(" ").collect();

        assert_eq!(map_entry_split.len(), 3);

        let destination_range_start = map_entry_split.get(0).unwrap().parse().unwrap();
        let source_range_start = map_entry_split.get(1).unwrap().parse().unwrap();
        let range_length = map_entry_split.get(2).unwrap().parse().unwrap();

        map_read.push(MapEntryDescription {
            source_range_start,
            source_range_end: source_range_start + range_length - 1, // -1 because start is inclusive
            destination_range_start,
            destination_range_end: destination_range_start + range_length - 1,
            range_length,
        });
    }

    return Some(Almanac {
        seeds,
        seed_to_soil_map: SourceToDestinationMap::new(
            MapType::SeedToSoil, maps.get(&MapType::SeedToSoil).unwrap().to_owned(),
        ),
        soil_to_fertilizier_map: SourceToDestinationMap::new(
            MapType::SoilToFertilizer, maps.get(&MapType::SoilToFertilizer).unwrap().to_owned(),
        ),
        fertizilier_to_water_map: SourceToDestinationMap::new(
            MapType::FertizilerToWater, maps.get(&MapType::FertizilerToWater).unwrap().to_owned(),
        ),
        water_to_light_map: SourceToDestinationMap::new(
            MapType::WaterToLight, maps.get(&MapType::WaterToLight).unwrap().to_owned(),
        ),
        light_to_temperature: SourceToDestinationMap::new(
            MapType::LightToTemperate, maps.get(&MapType::LightToTemperate).unwrap().to_owned(),
        ),
        temperature_to_humidity: SourceToDestinationMap::new(
            MapType::TemperatureToHumidity, maps.get(&MapType::TemperatureToHumidity)
                .unwrap()
                .to_owned(),
        ),
        humidity_to_location: SourceToDestinationMap::new(
            MapType::HumidityToLocation, maps.get(&MapType::HumidityToLocation).unwrap().to_owned(),
        ),
    });
}