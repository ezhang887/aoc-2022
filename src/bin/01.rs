fn sum_of_line<'a>(input: impl Iterator<Item = &'a str>) -> u32 {
    // ["1", "2", "3", "4"], -> 10
    input.map(|c| c.parse::<u32>().unwrap()).sum::<u32>()
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|s| s.lines())
        .map(sum_of_line)
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|s| s.lines())
        .map(sum_of_line)
        .collect();
    calories.sort();
    Some(calories.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
