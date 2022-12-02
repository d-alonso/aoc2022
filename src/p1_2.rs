use crate::{Error, Result};

//The Elves take turns writing down the number of Calories contained by the various meals, snacks,
// rations, etc. that they've brought with them, one item per line. Each Elf separates their own
// inventory from the previous Elf's inventory (if any) by a blank line.

// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
fn p(input: &str) -> Result<usize> {
    let mut sums_per_elf = input
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
        .collect::<Result<Vec<_>>>()?;
    sums_per_elf.sort_by(|a, b| b.cmp(a)); // Descending sort
    Ok(sums_per_elf
        .into_iter()
        .take(3)
        .sum())
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
    assert!(matches!(dbg!(p(input)), Ok(45000)));
}

#[test]
fn problem() {
    let input = include_str!("../inputs/p1.txt");
    assert!(matches!(dbg!(p(input)), Ok(206289)));
}
