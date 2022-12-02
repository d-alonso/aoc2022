use crate::{Error, Result};

//The Elves take turns writing down the number of Calories contained by the various meals, snacks,
// rations, etc. that they've brought with them, one item per line. Each Elf separates their own
// inventory from the previous Elf's inventory (if any) by a blank line.

// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
fn p(input: &str) -> Result<usize> {
    input
        .trim()
        .split("\n\n") // Split at empty lines to get elf inventories
        .map(|elf_str: &str| -> Result<usize> {
            Ok(elf_str
                .split('\n')
                .map(|calories_str| {
                    calories_str
                        .parse::<usize>()
                        .map_err(|e| Box::new(e) as Error)
                })
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .sum())
        })
        .collect::<Result<Vec<_>>>()? // Collect the sums per elf
        .into_iter()
        .max()
        .ok_or("No maximum in empty list".into())
}

#[test]
fn simple() {
    let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
    assert!(matches!(dbg!(p(input)), Ok(24000)));
}

#[test]
fn problem() {
    let input = include_str!("../inputs/p1.txt");
    assert!(matches!(dbg!(p(input)), Ok(71023)));
}
