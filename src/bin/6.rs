use std::fs;

struct Race {
    time: Vec<u64>,
    dist: Vec<u64>,
}

impl Race {
    fn new(value: &str) -> Self {
        let mut time = vec![];
        let mut dist = vec![];

        for line in value.split_terminator('\n') {
            if line.starts_with("Time:") {
                time = line[5..]
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
            } else if line.starts_with("Distance:") {
                dist = line[9..]
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
            } else {
                panic!();
            }
        }

        Race {
            time: time,
            dist: dist,
        }
    }

    // bad kerning; remove spaces between numbers. 7 15 30 -> 71530
    fn fold(&self) -> Self {
        let t = self
            .time
            .iter()
            .map(|x| x.to_string())
            .fold("".to_owned(), |a, b| a + &b)
            .parse::<u64>()
            .unwrap();
        let d = self
            .dist
            .iter()
            .map(|x| x.to_string())
            .fold("".to_owned(), |a, b| a + &b)
            .parse::<u64>()
            .unwrap();
        Race {
            time: vec![t],
            dist: vec![d],
        }
    }

    // for given input, output all possible distances
    fn get_race_dists(time: u64) -> Vec<u64> {
        (0..time + 1).map(|speed| speed * (time - speed)).collect()
    }

    fn get_wins(&self, race: usize) -> u64 {
        Race::get_race_dists(self.time[race])
            .iter()
            .filter(|dist| **dist > self.dist[race])
            .count()
            .try_into()
            .unwrap()
    }

    fn get_win_product(&self) -> u64 {
        (0..self.time.len())
            .map(|race_index| self.get_wins(race_index))
            .fold(1, |a, b| a * b)
    }
}

fn main() {
    let input = fs::read_to_string("6.input").expect("Should have been able to read the file");
    let race = Race::new(&input);

    println!("win product: {}", race.get_win_product());
    println!("win folded: {}", race.fold().get_win_product());
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn get_race() -> Race {
        let input =
            fs::read_to_string("6sample.input").expect("Should have been able to read the file");

        Race::new(&input)
    }

    #[test]
    fn test_new() {
        let race = get_race();

        assert_eq!(race.time, [7, 15, 30]);
        assert_eq!(race.dist, [9, 40, 200]);
    }

    #[test]
    fn test_get_race_dists() {
        let dists = Race::get_race_dists(7);
        assert_eq!(dists.len(), 8);

        assert_eq!(dists[0], 0);
        assert_eq!(dists[1], 6);
        assert_eq!(dists[2], 10);
        assert_eq!(dists[3], 12);
        assert_eq!(dists[4], 12);
        assert_eq!(dists[5], 10);
        assert_eq!(dists[6], 6);
        assert_eq!(dists[7], 0);
    }

    #[test]
    fn test_get_wins() {
        let race = get_race();

        assert_eq!(race.get_wins(0), 4);
        assert_eq!(race.get_wins(1), 8);
        assert_eq!(race.get_wins(2), 9);
    }

    #[test]
    fn test_get_win_product() {
        let race = get_race();

        assert_eq!(race.get_win_product(), 288);
    }

    #[test]
    fn test_fold() {
        let race = get_race().fold();

        assert_eq!(race.time[0], 71530);
        assert_eq!(race.dist[0], 940200);
    }

    #[test]
    fn test_folded_wins() {
        let race = get_race().fold();

        assert_eq!(race.get_win_product(), 71503);
    }
}
