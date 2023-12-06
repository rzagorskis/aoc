#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<i64>,

    pub seed_to_soil_map: SourceToDestinationMap,
    pub soil_to_fertilizier_map: SourceToDestinationMap,
    pub fertizilier_to_water_map: SourceToDestinationMap,
    pub water_to_light_map: SourceToDestinationMap,
    pub light_to_temperature: SourceToDestinationMap,
    pub temperature_to_humidity: SourceToDestinationMap,
    pub humidity_to_location: SourceToDestinationMap,
}

#[derive(Debug)]
pub struct SourceToDestinationMap {
    map_type: MapType,
    mappings: Vec<MapEntryDescription>
}

impl SourceToDestinationMap {
    pub fn new(map_type: MapType, mappings: Vec<MapEntryDescription>) -> Self {
        return SourceToDestinationMap { map_type, mappings }
    }

    pub fn destination(&self, source: i64) -> i64 {
        let mut destination: Option<i64> = None;
        for item in &self.mappings {
            if source >= item.source_range_start && source <= item.source_range_end {

                let offset_for_destination = ((item.source_range_start as i64) - (source as i64)).abs();
                destination = 
                    Some((item.destination_range_start) + offset_for_destination as i64);
                break;
            }
        }

        if destination.is_none() {
            destination = Some(source);
        }

        return destination.unwrap();
    }
}


#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertizilerToWater,
    WaterToLight,
    LightToTemperate,
    TemperatureToHumidity,
    HumidityToLocation
}

#[derive(Debug, Clone, Copy)]
pub struct MapEntryDescription {
    pub source_range_start: i64,
    pub source_range_end: i64,
    pub destination_range_start: i64,
    pub destination_range_end: i64,
    pub range_length: i64
}