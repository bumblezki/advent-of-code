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

fn count_minutes_asleep(events: &Vec<&SecurityEvent>) -> u32 {
    events.iter().filter(|event| event.event_type == SecurityEventType::Sleep).zip(
        events.iter().filter(|event| event.event_type == SecurityEventType::Wake)
    ).fold(0, |mut total, (sleep, wake)| {
        for _ in sleep.minute..wake.minute {
            total += 1;
        }
        total
    })
}

fn map_minutes_asleep(events: &Vec<&SecurityEvent>) -> HashMap<u32, u32> {
    events.iter().filter(|event| event.event_type == SecurityEventType::Sleep).zip(
        events.iter().filter(|event| event.event_type == SecurityEventType::Wake)
    ).fold(HashMap::new(), |mut map, (sleep, wake)| {
        for minute in sleep.minute..wake.minute {
            *map.entry(minute).or_insert(0) += 1;
        }
        map
    })
}

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {

    // Parse the input into a vector of all security events and sort these chronologically.
    let mut all_security_events: Vec<SecurityEvent> = input_lines[0].iter().map(|line | {
        SecurityEvent::from_input_line(line)
    }).collect();
    all_security_events.sort_by(|event1, event2| event1.cmp(event2));

    // Convert the vector of all security events into a HashMap where the key is the ID of a guard and the
    // value is a vector of all the security events associated with that guard that are of type Wake or Sleep.
    let mut guard_id: u32 = 0;
    let security_events_per_guard: HashMap<u32, Vec<&SecurityEvent>> = all_security_events
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
    // A HashMap where the key is the ID a guard and the value is a the total time they spent asleep while on shift.
    let total_minutes_asleep_per_guard = security_events_per_guard
        .iter().fold(HashMap::new(), |mut map, (&guard_id, events)| {
            map.insert(guard_id, count_minutes_asleep(events));
            map
        }
    );
    // A HashMap where the key is the ID a guard and the value is a vector of all the security events associated with
    // that guard in chronological order.
    let sleep_schedules_per_guard = &security_events_per_guard
        .iter().fold(HashMap::new(), |mut map, (&guard_id, events)| {
            map.insert(guard_id, map_minutes_asleep(events));
            map
        }
    );

    // Part 1
    let id_of_guard_with_most_total_minutes_asleep = total_minutes_asleep_per_guard
        .iter()
        .max_by(|&guard1_id_and_total, &guard2_id_and_total| guard1_id_and_total.1.cmp(&guard2_id_and_total.1))
        .map(|(id, _total)| id )
        .unwrap();
    let sleep_schedule_of_guard_with_most_total_minutes_asleep = sleep_schedules_per_guard.get(id_of_guard_with_most_total_minutes_asleep).unwrap();
    let sleepiest_minute_of_guard_with_most_total_minutes_asleep = sleep_schedule_of_guard_with_most_total_minutes_asleep
        .iter()
        .max_by(|&minute1_and_total, &minute2_and_total| minute1_and_total.1.cmp(&minute2_and_total.1))
        .map(|(minute, _total)| minute)
        .unwrap();
    let answer1 = id_of_guard_with_most_total_minutes_asleep * sleepiest_minute_of_guard_with_most_total_minutes_asleep;

    // Part 2
    let most_consistently_asleep_minute_and_count_per_guard = sleep_schedules_per_guard
        .iter()
        .fold(HashMap::new(), |mut map, (&guard_id, sleep_map)| {
            if let Some(most_consistently_asleep_minute_and_count) = sleep_map
                .iter()
                .max_by(|&a, &b| a.1.cmp(&b.1))
                .map(|(minute, count)| (minute, count)) {
                map.insert(guard_id, most_consistently_asleep_minute_and_count);
            }
            map
        }
    );
    let answer2 = most_consistently_asleep_minute_and_count_per_guard
        .iter()
        .max_by(|&(_, &guard1_minute_and_count), &(_, &guard2_minute_and_count)| guard1_minute_and_count.1.cmp(&guard2_minute_and_count.1))
        .map(|(id, &(minute, _count))| id * minute)
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