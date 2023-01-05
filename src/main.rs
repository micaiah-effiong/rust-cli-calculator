use std::env::{args, Args};

fn main() {
  let mut args:Args = args();

	// let first: String = args.nth(1).unwrap();
	// let operator: char = args.nth(0).unwrap().chars().next().unwrap();
	// let second: String = args.nth(0).unwrap();

	// let first_number:f32 = first.parse::<f32>().unwrap();
	// let second_number:f32 = second.parse::<f32>().unwrap();
	// let result:f32 = operate(operator, first_number,second_number);

	// println!("{:?}", output(first_number, operator, second_number, result));
	println!("{:?}, {}", args, args.len());

	let input = format!("{}", args.nth(1).unwrap());
	
	println!("{}", input);
	// println!("format_input_without_spaces({}) -> {:?}",input, format_input_without_spaces(input.as_str()) )
	let (first, operator, second) = format_input_without_spaces(input.as_str());
	// println!("{} {}", &first.parse::<f32>().unwrap(), &second);
	println!("{} = {}",input, operate(operator, first.parse::<f32>().unwrap(), second.parse::<f32>().unwrap()) );
}

fn format_input_without_spaces(input: &str) -> (&str, char, &str){
	let string_vector: Vec<char> = input.chars().collect();

	let mut operator: Option<char> = None;
	let mut index: Option<usize> = None;
	for (inx, item) in string_vector.iter().enumerate() {
			if is_operator(item) {
				operator = Some(item.to_owned());
				index = Some(inx);
				break;
			}
	}

	match operator {
    Some(_) => {},
    None => panic!("No valid operator was passed"),
	}

	match index {
    Some(_) => {},
    None => panic!("No valid operator was passed"),
	}

	let unwrapped_index: usize = index.unwrap();
	let first_part: &str = &input[0..unwrapped_index].trim();
	let second_part: &str = &input[unwrapped_index+1..].trim();

	println!("first {}", first_part);

	return (&first_part, operator.unwrap(), &second_part)
}

fn operate(operator: char, first_number:f32, second_number:f32) -> f32 {
	match operator {
		'+' => first_number + second_number,
		'-' => first_number - second_number,
		'/' => first_number / second_number,
		'*' | 'x' | 'X' => first_number * second_number,
		_ => panic!("Invalid operator used")
	}
}

fn is_operator(operator: &char) -> bool {
		match operator {
		'+' | '-' | '/' | '*' | 'x' | 'X' => true,
		_ => false
	}
}

fn output(first:f32, operator: char, second:f32, result:f32) -> String {
	format!("{} {} {} = {}", first, operator, second, result)
}