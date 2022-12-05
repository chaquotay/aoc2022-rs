use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Rucksack {
    first_compartment : String,
    second_compartment : String
}

impl Rucksack {
    fn common_item(self) -> char {
        let first_chars : HashSet<char> = self.first_compartment.chars().collect();
        let second_chars : HashSet<char> = self.second_compartment.chars().collect();

        return *first_chars.intersection(&second_chars).next().unwrap();
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(text : &str) -> Result<Rucksack, ()> {
        let parsed = Rucksack {
            first_compartment: text.get(..text.len()/2).unwrap().to_string(),
            second_compartment: text.get((text.len()/2)..).unwrap().to_string()
        };

        Ok(parsed)
    }
}

fn to_prio(c : char) -> i32 {
    match c {
        'a'..='z' => (c as i32)-96,
        'A'..='Z' => (c as i32)-38,
        _        => 0
    }
}

fn main() {
    let txt = include_str!("..\\input.txt");
    let sum : i32 = txt
        .lines()
        .map(|ln| ln.parse::<Rucksack>().unwrap())
        .map(|r| r.common_item())
        .map(|c| to_prio(c))
        .sum();
    println!("Sum: {}", sum);

    assert_eq!(sum, 8085);
}
