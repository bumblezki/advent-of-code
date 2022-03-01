use std::cmp::Ordering;
use std::iter::FromIterator;
use std::str::FromStr;
use chrono::prelude::*;
use dateparser::parse;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SecurityEventType {
    Wake,
    Sleep,
    StartShift(i32)
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct SecurityEvent {
    time: DateTime<Utc>,
    event_type: SecurityEventType
}

impl SecurityEvent {
    fn new(time: DateTime<Utc>, description: &str) -> Self {
        let event_type = match description {
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
        Self { time, event_type }
    }   

    fn from_input_line(input_line: &String) -> Self {
        // We can't parse dates before 1970 into a DateTime object.
        // So manipulate the strings to effectively add 1000 years to all each date.
        let mut input_line_chars: Vec<char> = input_line.clone().chars().collect();
        input_line_chars[1] = '2';
        let input_line_plus_1000y = String::from_iter(input_line_chars);
        
        let re = Regex::new(r"\[|\] ").unwrap();
        let split_line: Vec<&str> = re.split(&input_line_plus_1000y).collect();
        
        SecurityEvent::new(
            parse(split_line[1]).unwrap(),
            split_line[2]
        )
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

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    let mut answer1: Vec<SecurityEvent> = input_lines[0].iter().map(|line | {
        SecurityEvent::from_input_line(line)
    }).collect();
    answer1.sort_by(|event1, event2| event1.cmp(event2));
    let answer2 = 0;
    (format!("{:?}", answer1), format!("{}", answer2))
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