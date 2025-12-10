use std::{fs::File, io::{BufRead, BufReader}, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Rotate
{
    Left(u32),
    Right(u32),
}

impl FromStr for Rotate
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let prefix = chars.next().unwrap();
        let remainder: &str = &s[1..];
        let amount: u32 = remainder.parse().unwrap();
        Ok(match prefix
        {
            'L' => Self::Left(amount),
            'R' => Self::Right(amount),
			_ => panic!("Expected one of L or R"),
        })
    }
}


pub struct Safe
{
    current_dial_position: u32,
    dial_size: u32,
    zero_tap_count: u32,
}

impl Safe
{
    pub fn new(dial_size: u32, starting_point: u32) -> Self
    {
        Self
        {
            current_dial_position: starting_point,
            dial_size,
            zero_tap_count: 0,
        }
    }

    pub fn rotate(&mut self, rotate: Rotate)
    {
		println!("-----------------------");
		println!("Safe at {}", self.current_dial_position);
		println!("Rotating {:?}", rotate);
        self.current_dial_position = (match rotate
        {
            Rotate::Left(amount) => self.current_dial_position as i32 - amount as i32,
            Rotate::Right(amount) => self.current_dial_position as i32 + amount as i32,
        } + self.dial_size as i32) as u32 % self.dial_size;

        if self.current_dial_position == 0
        {
            self.zero_tap_count += 1;
			println!("Hit zero {} times!", self.zero_tap_count);
        }
		println!("Safe now at {}", self.current_dial_position);
    }
}

fn main()
{
	let mut safe = Safe::new(100, 50);
	let file = File::open("data/1.txt").unwrap();
	let reader = BufReader::new(file);
	let rotations: Vec<Rotate> = reader.lines().map(|line| Rotate::from_str(line.unwrap().as_str()).unwrap()).collect();
	for rotate in rotations
	{
		safe.rotate(rotate);
		/*
		let mut string = String::new();
		let mut handle = std::io::stdin().lock();
		handle.read_line(&mut string).unwrap();
		*/
	}

	println!("Times we stopped at 0: {}", safe.zero_tap_count);
}

#[cfg(test)]
mod tests
{
	use super::*;
	#[test]
	fn example() 
	{
		let mut safe = Safe::new(100, 50);
		let rotations = vec![
			Rotate::Left(68),
			Rotate::Left(30),
			Rotate::Right(48),
			Rotate::Left(5),
			Rotate::Right(60),
			Rotate::Left(55),
			Rotate::Left(1),
			Rotate::Left(99),
			Rotate::Right(14),
			Rotate::Left(82),
		];
		let expected_nums = vec![
			82,
			52,
			0,
			95,
			55,
			0,
			99,
			0,
			14,
			32];

		for (rotation, expected) in rotations.iter().zip(expected_nums.iter())
		{
			safe.rotate(*rotation);
			assert_eq!(safe.current_dial_position, *expected);
		}

		assert_eq!(safe.zero_tap_count, 3);
	}

	#[test]
	fn read_file()
	{
		let expected_rotations = vec![
			Rotate::Left(68),
			Rotate::Left(30),
			Rotate::Right(48),
			Rotate::Left(5),
			Rotate::Right(60),
			Rotate::Left(55),
			Rotate::Left(1),
			Rotate::Left(99),
			Rotate::Right(14),
			Rotate::Left(82),
		];

		let string = String::from("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n");
		let reader = BufReader::new(string.as_bytes());
		let rotations: Vec<Rotate> = reader.lines().map(|line| Rotate::from_str(line.unwrap().as_str()).unwrap()).collect();
		assert_eq!(rotations, expected_rotations);
	}

}
