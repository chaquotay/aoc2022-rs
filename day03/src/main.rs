use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Rucksack {
    first_compartment : String,
    second_compartment : String
}

impl Rucksack {
    fn common_item(&self) -> char {
        let first_chars : HashSet<char> = self.first_compartment.chars().collect();
        let second_chars : HashSet<char> = self.second_compartment.chars().collect();

        return *first_chars.intersection(&second_chars).next().unwrap();
    }

    fn all(&self) -> HashSet<char> {
        let mut all : HashSet<char> = HashSet::new();
        all.extend(self.first_compartment.chars());
        all.extend(self.second_compartment.chars());
        all
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(text : &str) -> Result<Rucksack, ()> {
        let split = text.len()/2;
        let parsed = Rucksack {
            first_compartment: text.get(..split).unwrap().to_string(),
            second_compartment: text.get(split..).unwrap().to_string()
        };

        Ok(parsed)
    }
}

fn to_prio(c : char) -> i32 {
    match c {
        'a'..='z' => (c as i32)-96,
        'A'..='Z' => (c as i32)-38,
        _         => 0
    }
}

fn main() {
    let txt = include_str!("..\\input.txt");
    let rucksacks : Vec<Rucksack> = txt
        .lines()
        .map(|ln| ln.parse::<Rucksack>().unwrap())
        .collect();

    let sum : i32 = rucksacks
        .iter()
        .map(|r| r.common_item())
        .map(|c| to_prio(c))
        .sum();
    println!("Sum: {}", sum);

    let sum2 : i32 = rucksacks
        .chunks(3)
        .map(|g| g.into_iter().map(|x| x.all()).reduce(|acc,x| acc.intersection(&x).map(|c| *c).collect()))
        .map(|c| to_prio(c.unwrap().into_iter().next().unwrap()))
        .sum();
    println!("Sum2: {}", sum2);

    assert_eq!(sum, 8085);
}
