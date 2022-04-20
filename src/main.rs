use std::collections::VecDeque;

fn infix_postfix(input: String) -> String {
	let mut stack = VecDeque::new();
	let mut output = String::new();

	for c in input.chars() {
		// if an operand, push to output
		if c.is_alphanumeric() {
			output.push(c);
		}
		// if open p, push to stack	
		else if c == '(' {
			stack.push_front(c);
		}
		// closing p, pop operators to output until open p
		else if c == ')' {
			while stack.len() != 0 {
				match stack.pop_front() {
					None => println!("stack is empty, {}", output),
					Some(val) => {
						if val != '(' {
							output.push(val);
						} else {
							break;
						}
					},
				}
			}
		}
		// its a operator, pop operators to output until a lower prec is reached
		else {
			while stack.len() != 0 {
				let f = stack.front().expect("some");
				if prec(*f) >= prec(c) {
					output.push(stack.pop_front().unwrap());
				} else {
					break;
				}
			}
			stack.push_front(c);
		}
	}
	// output all remaining operators to out
	while stack.len() > 0 {
		output.push(stack.pop_front().expect("Errorr"));
	}

	for i in stack {
		println!("{}", i);
	}

	output
}

fn prec(c: char) -> i32 {
	if c == '^' {
		3
	} else if c == '/' || c == '*' { 
		2
	} else if c == '-' || c == '+' {
		1
	} else {
		-1
	}
}
	

fn main() {
	let infix_exp = String::from("a/(b+c)-d");
	println!("{}", infix_exp);	

	let post = infix_postfix(infix_exp);
	println!("{}", post);
}
