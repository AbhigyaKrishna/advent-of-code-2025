use advent_of_code::MinHeap;
use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();

    let mut heap1: MinHeap = MinHeap::new();
    let mut heap2: MinHeap = MinHeap::new();

    let mut count = 0;
    for line in lines {
        let mut split = line.split("   ");
        let num1 = split.next()?.parse::<i32>().unwrap();
        let num2 = split.next()?.parse::<i32>().unwrap();

        heap1.push(num1);
        heap2.push(num2);
        count += 1;
    }

    let mut sum = 0;
    for _ in 0..count {
        let dist = (heap1.pop().unwrap() - heap2.pop().unwrap()).abs();
        sum += dist as u64;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0u64;

    let mut left_count = HashMap::<u32, usize>::new();
    let mut right_count = HashMap::<u32, usize>::new();
    for line in input.lines() {
        let mut split = line.split("   ");
        let num1 = split.next()?.parse::<u32>().unwrap();
        let num2 = split.next()?.parse::<u32>().unwrap();

        sum += (*left_count.get(&num2).unwrap_or(&0) * num2 as usize) as u64;
        sum += (*right_count.get(&num1).unwrap_or(&0) * num1 as usize) as u64;

        if num1 == num2 {
            sum += num1 as u64;
        }

        left_count.entry(num1).and_modify(|e| *e += 1).or_insert(1);
        right_count.entry(num2).and_modify(|e| *e += 1).or_insert(1);
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
