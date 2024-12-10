aoc2024::solution!(1);

#[derive(Debug)]
struct Pairs {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn parse_file(input: &str) -> Pairs {
    let mut left = Vec::new();
    let mut right = Vec::new();

    input.split("\n").for_each(|line| {
        if line == "" {
            return;
        }
        let mut pair = line.split("   ");
        // println!("{:?}", pair);
        left.push(pair.next().unwrap().parse::<i32>().unwrap());
        right.push(pair.next().unwrap().parse::<i32>().unwrap());
    });

    let map = Pairs {
        left: left,
        right: right,
    };

    return map;
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut pairs = parse_file(input);

    pairs.left.sort();
    pairs.right.sort();

    let mut delta: i32 = 0;
    let mut x = 0;
    while x < pairs.left.len() {
        let l = pairs.left[x];
        let r = pairs.right[x];
        delta = delta + (l-r).abs();
        x = x+1;
    }
    // pairs.left.iter().for_each(|x| {

    // })

    return Some(delta);
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut pairs = parse_file(input);

    pairs.left.sort();
    pairs.right.sort();

    let mut total = 0;
    let mut x = 0;
    let mut y = 0;
    let mut current = 0;
    let mut count = 0;
    while x < pairs.left.len() {
        let next = pairs.left[x];
        if current != next {
            count = 0;
            current = next;
        }
        while y < pairs.right.len() {
            if pairs.right[y] == current {
                count += 1;
            } else if pairs.right[y] > current {
                break;
            }

            y += 1;
        }
        total += count * current;
        x += 1;
    }

    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
