advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once("   ")?;
            let left: u32 = left.parse().ok()?;
            let right: u32 = right.parse().ok()?;

            Some((left, right))
        })
        .unzip();

    left_list.sort();
    right_list.sort();

    let res = left_list
        .into_iter()
        .zip(right_list)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once("   ")?;
            let left: u32 = left.parse().ok()?;
            let right: u32 = right.parse().ok()?;

            Some((left, right))
        })
        .unzip();

    let res = left_list
        .iter()
        .map(|left| left * right_list.iter().filter(|right| left == *right).count() as u32)
        .sum();

    Some(res)
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
