use advent_of_code::MinHeap;

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
    None
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
        assert_eq!(result, None);
    }
}
