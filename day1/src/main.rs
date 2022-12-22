use std::{fs, iter::Sum, path::Path};

fn main() -> Result<(), std::io::Error> {
    let file_path = "C:\\Users\\do.tran\\Desktop\\advent-of-code-2022\\day1\\src\\calories.txt";

    let file_content = read_file_content(file_path)?;

    let chunks = split_file_content_into_chunk(&file_content);

    // let mut elves: Vec<Elf> = chunks.into_iter().map(|chunk| chunk.into()).collect();
    // elves.sort_by(|a, b| b.cmp(a));
    // dbg!(elves.first());

    let mut elves: Vec<Elf> = chunks.into_iter().map(|x| x.into()).collect();
    elves.sort_by(|a, b| b.cmp(a));
    let sum: Elf = elves.into_iter().take(3).sum();

    println!("{:?}", &sum);

    Ok(())
}

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
struct Elf(i32);

impl From<&str> for Elf {
    fn from(value: &str) -> Self {
        let vec: Vec<i32> = value
            .trim()
            .split("\r\n")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let total = vec.iter().sum();

        Self(total)
    }
}
impl Sum for Elf {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut sum = 0;
        loop {
            if let Some(item) = iter.next() {
                sum += item.0;
            } else {
                break;
            }
        }

        Elf(sum)
    }
}

fn parse_file_to_elf(chunk: &str) -> Vec<Elf> {
    todo!()
}

fn read_file_content<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let file_content = fs::read_to_string(path)?;
    return Ok(file_content);
}

fn split_file_content_into_chunk(file_content: &str) -> Vec<&str> {
    let chunk: Vec<&str> = file_content.split("\r\n\r\n").collect();
    chunk
}
