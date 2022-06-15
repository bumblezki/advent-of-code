// Potential improvements:
//

use itertools::all;
use regex::Regex;
use std::collections::{BTreeMap, HashSet};

fn secs(task: char) -> i32 {
    task as i32 - 4
}

#[derive(Clone)]
struct Graph {
    edges: BTreeMap<char, HashSet<char>>,
    queue: Vec<char>,
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
        Graph {
            edges,
            queue: Vec::new(),
        }
    }

    fn complete(&mut self, step: &char) {
        self.edges.remove(&step);
        for prereqs in self.edges.values_mut() {
            prereqs.remove(&step);
        }
        println!("Completed: {}", step);
    }

    fn queue_completed_steps(&mut self, in_progress: &Vec<Option<char>>) {
        for (&step, prereqs) in self.edges.iter() {
            if !self.queue.contains(&step)
                && prereqs.is_empty()
                && !in_progress.contains(&Some(step))
            {
                self.queue.push(step.clone());
                self.queue.sort();
                self.queue.reverse();
            }
        }
        println!("Queue: {}", String::from_iter(self.queue.clone()));
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (step, prereqs) in self.edges.iter() {
            writeln!(f, "{}  <-  {:?}", step, prereqs)?;
        }
        Ok(())
    }
}

pub fn day07(input_lines: &[Vec<String>]) -> (String, String) {
    let graph = Graph::from_input(&input_lines[0]);
    let mut order1: Vec<char> = Vec::new();
    let mut graph1 = graph.clone();
    graph1.queue_completed_steps(&vec![]);
    loop {
        // println!("{}", graph);
        match graph1.queue.pop() {
            Some(step) => {
                graph1.complete(&step);
                graph1.queue_completed_steps(&vec![]);
                order1.push(step);
            }
            None => break,
        }
    }
    let answer1 = String::from_iter(order1);

    let mut graph2 = graph.clone();
    let mut workers = vec![0, 0, 0, 0, 0];
    let mut tasks: Vec<Option<char>> = vec![None, None, None, None, None];
    let mut counter = 0;
    loop {
        graph2.queue_completed_steps(&tasks);

        for worker in 0..5 {
            if workers[worker] == 0 && tasks[worker].is_some() {
                graph2.complete(&tasks[worker].unwrap());
                graph2.queue_completed_steps(&tasks);
                tasks[worker] = None;
            }
        }

        for worker in 0..5 {
            if workers[worker] == 0 && tasks[worker].is_none() {
                if let Some(next_task) = graph2.queue.pop() {
                    tasks[worker] = Some(next_task);
                    workers[worker] = secs(next_task)
                }
            }
        }

        for worker in 0..5 {
            if workers[worker] != 0 {
                workers[worker] -= 1;
            }
        }

        println!("{:?}", workers);
        println!("{:?}", tasks);
        println!("{}", counter);
        if graph2.queue.is_empty() && all(&tasks, |&task| task.is_none()) {
            break;
        } else {
            counter += 1;
        }
    }
    let answer2 = counter;
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
            "0",      // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day07(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
