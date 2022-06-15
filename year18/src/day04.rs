use chrono::prelude::*;
use counter::Counter;
use dateparser;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum SecurityEventType {
    Wake,
    Sleep,
    StartShift(u32),
}

impl fmt::Display for SecurityEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match &self {
            Self::Wake => "Wake".to_string(),
            Self::Sleep => "Sleep".to_string(),
            Self::StartShift(id) => format!("Guard {}: Start Shift", id),
        };
        write!(f, "{}", description)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct SecurityEvent {
    time: DateTime<Utc>,
    minute: u32,
    event_type: SecurityEventType,
}

impl SecurityEvent {
    fn new(time: DateTime<Utc>, event_type: SecurityEventType) -> Self {
        let minute = time.minute();
        Self {
            time,
            minute,
            event_type,
        }
    }

    fn from_input_line(input_line: &String) -> Self {
        // We can't parse dates before 1970 into a DateTime object.
        // The actual year doesn't matter, only the order and the minute matter.
        // So manipulate the strings to effectively add 1000 years to all each date.
        let mut input_line_chars: Vec<char> = input_line.clone().chars().collect();
        input_line_chars[1] = '2';
        let input_line_plus_1000y = String::from_iter(input_line_chars);

        // Split the line on either "[" or "] ".
        // E.g., "[1518-10-09 00:56] wakes up" goes to ['', '2518-10-09 00:56', 'wakes up'].
        let re = Regex::new(r"\[|\] ").unwrap();
        let split_line: Vec<&str> = re.split(&input_line_plus_1000y).collect();
        let date_str = split_line[1];
        let event_str = split_line[2];

        // Parse the str type date into a DateTime object.
        let time = dateparser::parse(date_str).unwrap();

        // Parse the str type event into a SecurityEventType object.
        let event_type = match event_str {
            "wakes up" => SecurityEventType::Wake,
            "falls asleep" => SecurityEventType::Sleep,
            description => {
                let re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
                re.captures(description)
                    .map(|cap| {
                        let id = FromStr::from_str(&cap[1]).unwrap();
                        SecurityEventType::StartShift(id)
                    })
                    .expect("Failed to capture regular expression from security event.")
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

fn guard_records_valid(events: &Vec<SecurityEvent>) -> bool {
    events.chunks(2).all(|pair| {
        pair[0].event_type == SecurityEventType::Sleep
            && pair[1].event_type == SecurityEventType::Wake
    })
}

fn count_minutes_asleep(events: &Vec<SecurityEvent>) -> u32 {
    events.chunks(2).fold(0, |mut total, pair| {
        total += pair[1].minute - pair[0].minute;
        total
    })
}

fn map_minutes_asleep(events: &Vec<SecurityEvent>) -> Counter<u32> {
    events.chunks(2).fold(Counter::new(), |mut counter, pair| {
        counter.extend((pair[0].minute..pair[1].minute).collect::<Counter<u32>>());
        counter
    })
}

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    // Parse the input into a vector of all security events and sort these chronologically.
    let mut all_security_events: Vec<SecurityEvent> = input_lines[0]
        .iter()
        .map(|line| SecurityEvent::from_input_line(line))
        .collect();
    all_security_events.sort_by(|event1, event2| event1.cmp(event2));

    // Convert the vector of all security events into a HashMap where the key is the ID of a
    // guard and the value is a vector of all the Wake and Sleep security events associated
    // with that guard.
    let mut guard_id: u32 = 0;
    let mut security_events_per_guard = HashMap::<u32, Vec<SecurityEvent>>::new();
    for security_event in all_security_events {
        if let SecurityEventType::StartShift(id) = security_event.event_type {
            guard_id = id;
            if !security_events_per_guard.contains_key(&guard_id) {
                security_events_per_guard.insert(guard_id, Vec::new());
            }
        } else {
            security_events_per_guard
                .entry(guard_id)
                .and_modify(|events| events.push(security_event));
        }
    }
    for (guard_id, events) in &security_events_per_guard {
        if !guard_records_valid(events) {
            panic!(
                "The records for security guard {} must alternate between Sleep and Wake.",
                guard_id
            );
        }
    }

    // A HashMap where the key is the ID a guard and the value is a the total time they spent
    // asleep while on shift.
    let mut total_minutes_asleep_per_guard = HashMap::<u32, u32>::new();
    // A HashMap where the key is the ID a guard and the value is a vector of all the security
    // events associated with that guard in chronological order.
    let mut sleep_schedules_per_guard = HashMap::<u32, Counter<u32>>::new();
    for (guard_id, security_events) in security_events_per_guard {
        total_minutes_asleep_per_guard.insert(guard_id, count_minutes_asleep(&security_events));
        sleep_schedules_per_guard.insert(guard_id, map_minutes_asleep(&security_events));
    }

    // Part 1
    let id_of_guard_with_most_total_minutes_asleep = total_minutes_asleep_per_guard
        .iter()
        .max_by(
            |&(_this_guard_id, &this_guard_total), &(&_that_guard_id, &that_guard_total)| {
                this_guard_total.cmp(&that_guard_total)
            },
        )
        .map(|(&guard_id, _guard_total)| guard_id)
        .expect("Failed to find the ID of the guard who spent the most time asleep.");
    let sleep_schedule_of_guard_with_most_total_minutes_asleep = sleep_schedules_per_guard
        .get(&id_of_guard_with_most_total_minutes_asleep)
        .unwrap();
    let sleepiest_minute_of_guard_with_most_total_minutes_asleep =
        sleep_schedule_of_guard_with_most_total_minutes_asleep
            .iter()
            .max_by(
                |&(_this_minute, &this_total), &(&_that_minute, &that_total)| {
                    this_total.cmp(&that_total)
                },
            )
            .map(|(&minute, _total)| minute)
            .expect(
                "Failed to find the ID of the guard who was asleep on the same minute the most.",
            );
    let answer1 = id_of_guard_with_most_total_minutes_asleep
        * sleepiest_minute_of_guard_with_most_total_minutes_asleep;

    // Part 2
    let mut most_consistently_asleep_minute_and_count_per_guard =
        HashMap::<u32, (u32, usize)>::new();
    for (guard_id, sleep_schedule) in sleep_schedules_per_guard {
        if let Some(most_consistently_asleep_minute_and_count) = sleep_schedule
            .iter()
            .max_by(
                |&(_this_minute, &this_count), &(_that_minute, &that_count)| {
                    this_count.cmp(&that_count)
                },
            )
            .map(|(&minute, &count)| (minute, count))
        {
            most_consistently_asleep_minute_and_count_per_guard
                .insert(guard_id, most_consistently_asleep_minute_and_count);
        }
    }

    let answer2 = most_consistently_asleep_minute_and_count_per_guard
        .iter()
        .max_by(
            |&(_this_guard_id, &(_this_guard_minute, this_guard_count)),
             &(_that_guard_id, &(_that_guard_minute, that_guard_count))| {
                this_guard_count.cmp(&that_guard_count)
            },
        )
        .map(|(&guard_id, &(minute, _count))| guard_id * minute)
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
            "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up", // INPUT STRING
            "240",  // PART 1 RESULT
            "4455", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day04(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
