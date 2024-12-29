use crate::FlowDirection::{Ascending, Descending, Undetermined};

advent_of_code::solution!(2);

enum FlowDirection {
    None,
    Undetermined(i32),
    Ascending(i32),
    Descending(i32),
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0u64;
    'outer: for line in input.lines() {
        let mut dir = FlowDirection::None;
        for nums in line.split(' ') {
            let num = nums.parse::<i32>().unwrap();
            match dir {
                FlowDirection::None => dir = Undetermined(num),
                Undetermined(prev) => {
                    if prev == num || (prev - num).abs() > 3 {
                        continue 'outer;
                    }

                    dir = if prev < num {
                        Ascending(num)
                    } else {
                        Descending(num)
                    }
                }
                Ascending(prev) => {
                    if prev == num || prev > num || num - prev > 3 {
                        continue 'outer;
                    }
                    dir = Ascending(num)
                }
                Descending(prev) => {
                    if prev == num || prev < num || prev - num > 3 {
                        continue 'outer;
                    }
                    dir = Descending(num)
                }
            }
        }
        count += 1;
    }

    Some(count)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
