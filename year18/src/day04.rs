use std::cmp::Ordering;
use std::iter::FromIterator;
use std::str::FromStr;
use chrono::prelude::*;
use dateparser;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum SecurityEventType {
    Wake,
    Sleep,
    StartShift(u32)
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct SecurityEvent {
    time: DateTime<Utc>,
    minute: u32,
    event_type: SecurityEventType
}

impl SecurityEvent {
    fn new(time: DateTime<Utc>, event_type: SecurityEventType) -> Self {
        let minute = time.minute();
        Self { time, minute, event_type }
    }

    fn from_input_line(input_line: &String) -> Self {
        // We can't parse dates before 1970 into a DateTime object.
        // So manipulate the strings to effectively add 1000 years to all each date.
        let mut input_line_chars: Vec<char> = input_line.clone().chars().collect();
        input_line_chars[1] = '2';
        let input_line_plus_1000y = String::from_iter(input_line_chars);
        
        let re = Regex::new(r"\[|\] ").unwrap();
        let split_line: Vec<&str> = re.split(&input_line_plus_1000y).collect();
        
        let time = dateparser::parse(split_line[1]).unwrap();
        let event_type = match split_line[2] {
            "wakes up" => SecurityEventType::Wake,
            "falls asleep" => SecurityEventType::Sleep,
            description => {
                let re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
                re.captures(description).map(|cap| {
                    let id = FromStr::from_str(&cap[1]).unwrap();
                    SecurityEventType::StartShift(id)
                }).unwrap()
            }
        };
        SecurityEvent::new(time, event_type)
    }
}

impl Ord for SecurityEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.time < other.time {
            return Ordering::Less;
        }
        if self.time > other.time {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

fn count_minutes_asleep(records: &Vec<&SecurityEvent>) -> u32 {
    records.iter().filter(|event| event.event_type == SecurityEventType::Sleep).zip(
        records.iter().filter(|event| event.event_type == SecurityEventType::Wake)
    ).fold(0, |mut total, (sleep, wake)| {
        for _ in sleep.minute..wake.minute {
            total += 1;
        }
        total
    })
}

fn map_minutes_asleep(records: &Vec<&SecurityEvent>) -> HashMap<u32, u32> {
    records.iter().filter(|event| event.event_type == SecurityEventType::Sleep).zip(
        records.iter().filter(|event| event.event_type == SecurityEventType::Wake)
    ).fold(HashMap::new(), |mut map, (sleep, wake)| {
        for minute in sleep.minute..wake.minute {
            *map.entry(minute).or_insert(0) += 1;
        }
        map
    })
}

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    // Part 1
    let mut security_events: Vec<SecurityEvent> = input_lines[0].iter().map(|line | {
        SecurityEvent::from_input_line(line)
    }).collect();
    security_events.sort_by(|event1, event2| event1.cmp(event2));

    let mut guard_id: u32 = 0;
    let per_guard_records: HashMap<u32, Vec<&SecurityEvent>> = security_events
        .iter().fold(HashMap::new(), |mut hashmap, event| {
            if let SecurityEventType::StartShift(id) = event.event_type {
                guard_id = id;
                if !hashmap.contains_key(&guard_id) {
                    hashmap.insert(guard_id, Vec::new());
                }
            } else {
                hashmap.entry(guard_id).and_modify(|events| events.push(event));
            }
            hashmap
        }
    );
    let guards_minutes_asleep = per_guard_records
        .iter().fold(HashMap::new(), |mut map, (&guard_id, record)| {
            map.insert(guard_id, count_minutes_asleep(record));
            map
        }
    );
    let sleepiest_guard = *guards_minutes_asleep
        .iter()
        .max_by(|&a, &b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();
    let sleepiest_guard_records = per_guard_records.get(&sleepiest_guard).unwrap();
    let sleepiest_guard_map = map_minutes_asleep(sleepiest_guard_records);
    let sleepiest_minute = *sleepiest_guard_map
        .iter()
        .max_by(|&a, &b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();
    let answer1 = sleepiest_guard * sleepiest_minute;

    // Part 2
    let guards_sleep_maps = &per_guard_records
        .iter().fold(HashMap::new(), |mut map, (&guard_id, record)| {
            map.insert(guard_id, map_minutes_asleep(record));
            map
        }
    );
    let guard_most_asleep_minutes = guards_sleep_maps.iter().fold(HashMap::new(), |mut map, (&guard_id, sleep_map)| {
        if let Some(minute_most_asleep) = sleep_map
            .iter()
            .max_by(|&a, &b| a.1.cmp(&b.1))
            .map(|(k, v)| (k, v)) {
            map.insert(guard_id, minute_most_asleep);
        }
        map
    });
    let answer2 = guard_most_asleep_minutes
        .iter()
        .max_by(|&(_, &a), &(_, &b)| a.1.cmp(&b.1))
        .map(|(k, &(minute, _))| k * minute)
        .unwrap();

    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day04;
    use crate::utils::load_input;

    #[test]
    fn check_day04_case01() {
        full_test(
"", // INPUT STRING
"0", // PART 1 RESULT
"0" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day04(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}