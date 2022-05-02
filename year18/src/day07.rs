// Potential improvements:
//

use std::collections::{BTreeMap, HashSet};
use regex::Regex;

struct Graph {
    edges: BTreeMap<char, HashSet<char>>,
}

impl Graph {
    fn from_input(input_lines: &Vec<String>) -> Self {
        let mut edges = BTreeMap::<char, HashSet<char>>::new();
        let mut steps = HashSet::<char>::new();
        let re = Regex::new(r"Step | must be finished before step | can begin.").unwrap();
        for line in input_lines {
            let chars: Vec<char> = re.replace_all(line, "").chars().collect();
            let u: char = chars[0];
            let v: char = chars[1];
            steps.insert(u);
            steps.insert(v);
            let requirements = edges.entry(v).or_insert(HashSet::new());
            requirements.insert(u);
        }
        for step in steps {
            if !edges.contains_key(&step) {
                edges.insert(step, HashSet::<char>::new());
            }
        }
        Graph { edges }
    }

    fn complete(&mut self, step: &char) {
        self.edges.remove(&step);
        for prereqs in self.edges.values_mut() {
            prereqs.remove(&step);
        }
        println!("Completed: {}", step);
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (step, prereqs) in self.edges.iter() {
            writeln!(f, "{}:     {:?}", step, prereqs)?;
        }
        Ok(())
    }
}

fn queue_completed_steps(q: &mut Vec<char>, graph: &mut Graph) {
    for (step, prereqs) in graph.edges.iter() {
        if !q.contains(step) && prereqs.is_empty() {
            q.push(step.clone());
            q.sort();
            q.reverse();
        }
    }
}

pub fn day07(input_lines: &[Vec<String>]) -> (String, String) {
    let mut graph = Graph::from_input(&input_lines[0]);
    let mut order: Vec<char> = Vec::new();
    let mut q: Vec<char> = Vec::new();
    queue_completed_steps(&mut q, &mut graph);
    loop {
        println!("{}", graph);
        println!("{:?}", q);
        match q.pop() {
            Some(step) => {
                graph.complete(&step);
                queue_completed_steps(&mut q, &mut graph);
                order.push(step);
            },
            None => break,
        }
    }
    let answer1 = String::from_iter(order);
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day07;
    use crate::utils::load_input;

    #[test]
    fn check_day07_case01() {
        full_test(
"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.", // INPUT STRING
"CABDFE", // PART 1 RESULT
"0" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day07(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}