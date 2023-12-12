use std::str::FromStr;

#[derive(Debug)]
struct Mapping {
    to: u64,
    from: u64,
    size: u64,
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap());
        Ok(Mapping {
            to: nums.next().unwrap(),
            from: nums.next().unwrap(),
            size: nums.next().unwrap(),
        })
    }
}

impl Mapping {
    fn contains(&self, val: &u64) -> bool {
        &self.from <= val && val < &(self.from + self.size)
    }

    fn map(&self, val: &u64) -> Option<u64> {
        if self.contains(val) {
            Some(self.to + val - self.from)
        } else {
            None
        }
    }
}

fn follow_mapping(input: &u64, mappings: &[Mapping]) -> u64 {
    for mapping in mappings {
        if let Some(output) = mapping.map(input) {
            // println!("{} -> {} via {:?}", input, output, mapping);
            return output;
        }
    }
    // println!("{} -> {}", input, input);
    *input
}

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    let mut lines = input_lines.iter();

    let seeds = lines.next().unwrap()[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let seeds_to_soil = lines
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|s| s.parse::<Mapping>().expect("Failed to parse Mapping"))
        .collect::<Vec<Mapping>>();
    let soil_to_fertilizer = lines
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|s| s.parse::<Mapping>().expect("Failed to parse Mapping"))
        .collect::<Vec<Mapping>>();
    let fertilizer_to_water = lines
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|s| s.parse::<Mapping>().expect("Failed to parse Mapping"))
        .collect::<Vec<Mapping>>();
    let water_to_light = lines
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|s| s.parse::<Mapping>().expect("Failed to parse Mapping"))
        .collect::<Vec<Mapping>>();
    let light_to_temperature = lines
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|s| s.parse::<Mapping>().expect("Failed to parse Mapping"))
        .collect::<Vec<Mapping>>();
    let temperature_to_humidity = lines
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|s| s.parse::<Mapping>().expect("Failed to parse Mapping"))
        .collect::<Vec<Mapping>>();
    let humidity_to_location = lines
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|s| s.parse::<Mapping>().expect("Failed to parse Mapping"))
        .collect::<Vec<Mapping>>();

    

    let answer1 = seeds.iter().map(|seed| {
        let soil = follow_mapping(seed, &seeds_to_soil);
        let fertilizer = follow_mapping(&soil, &soil_to_fertilizer);
        let water = follow_mapping(&fertilizer, &fertilizer_to_water);
        let light = follow_mapping(&water, &water_to_light);
        let temperature = follow_mapping(&light, &light_to_temperature);
        let humidity = follow_mapping(&temperature, &temperature_to_humidity);
        let location = follow_mapping(&humidity, &humidity_to_location);
        // println!();
        location
    }).min().unwrap();
    let answer2 = seeds.chunks(2).map(|chunk| {
        let mut lowest_location = u64::MAX;
        for seed in chunk[0]..chunk[0]+chunk[1] {
            let soil = follow_mapping(&seed, &seeds_to_soil);
            let fertilizer = follow_mapping(&soil, &soil_to_fertilizer);
            let water = follow_mapping(&fertilizer, &fertilizer_to_water);
            let light = follow_mapping(&water, &water_to_light);
            let temperature = follow_mapping(&light, &light_to_temperature);
            let humidity = follow_mapping(&temperature, &temperature_to_humidity);
            let location = follow_mapping(&humidity, &humidity_to_location);
            // println!();
            if location < lowest_location {
                lowest_location = location;
            }
        }
        lowest_location
    }).min().unwrap();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day05;
    use crate::utils::load_input;

    #[test]
    fn check_day05_case01() {
        full_test(
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",  // INPUT STRING
            "35", // PART 1 RESULT
            "46", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day05(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
