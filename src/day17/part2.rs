#[derive(Debug, Clone, Copy)]
struct Registers {
  a: i64,
  b: i64,
  c: i64,
}

pub fn solution() -> Vec<i64> {
  let input: String = std::fs::read_to_string("src/inputs/day17.txt")
    .expect("failed to parse input")
    .trim()
    .to_string();
  let stuff = parse_input(&input);
  let mut registers = stuff.0;
  let instructions: Vec<i64> = stuff.1;

  let mut pointer: i64 = 0;

  let mut output: Vec<i64> = vec![];
  let mut seq: Vec<i64> = Vec::new();

  loop {
    seq.push(registers.a);
    if execute(&mut registers, &instructions, &mut pointer, &mut output) == false {
      break;
    }
  }
  seq.dedup();
  println!("{:?}", seq);
  println!("{:?}", output);
  return output;
}

fn execute(
  registers: &mut Registers,
  instructions: &Vec<i64>,
  pointer: &mut i64,
  output: &mut Vec<i64>,
) -> bool {
  let index_option = instructions.get(*pointer as usize);
  if let Some(&index) = index_option {
    match index {
      0 => {
        let numerator = registers.a as f64;
        let combo_value_option = get_combo_value(registers, instructions, pointer);
        if let Some(combo_value) = combo_value_option {
          let denominator = 2.0_f64.powi(combo_value as i32);
          let result = numerator / denominator;
          let str_result = result.to_string();
          let truncated_result = str_result.split(".").collect::<Vec<&str>>()[0]
            .parse::<i64>()
            .expect("failed to parse str_result to int");
          registers.a = truncated_result;
          *pointer += 2;
          return true;
        } else {
          return false;
        }
      }
      1 => {
        let result = registers.b ^ instructions[*pointer as usize + 1];
        registers.b = result;
        *pointer += 2;
        return true;
      }
      2 => {
        let combo_value_option = get_combo_value(registers, instructions, pointer);
        if let Some(combo_value) = combo_value_option {
          let result = combo_value % 8;
          registers.b = result;
          *pointer += 2;
          return true;
        } else {
          return false;
        }
      }
      3 => {
        if registers.a == 0 {
          *pointer += 2;
          return true;
        } else {
          let pointer_option = instructions.get(*pointer as usize + 1);
          if let Some(&val) = pointer_option {
            *pointer = val;
            return true;
          } else {
            return false;
          }
        }
      }
      4 => {
        let result = registers.b ^ registers.c;
        registers.b = result;
        *pointer += 2;
        return true;
      }
      5 => {
        let combo_value_option = get_combo_value(registers, instructions, pointer);
        if let Some(combo_value) = combo_value_option {
          let result = combo_value % 8;
          output.push(result);
          *pointer += 2;
          return true;
        } else {
          return false;
        }
      }
      6 => {
        let numerator = registers.a as f64;
        let combo_value_option = get_combo_value(registers, instructions, pointer);
        if let Some(combo_value) = combo_value_option {
          let denominator = 2.0_f64.powi(combo_value as i32);
          let result = numerator / denominator;
          let str_result = result.to_string();
          let truncated_result = str_result.split(".").collect::<Vec<&str>>()[0]
            .parse::<i64>()
            .expect("failed to parse str_result to int");
          registers.b = truncated_result;
          *pointer += 2;
          return true;
        } else {
          return false;
        }
      }
      7 => {
        let numerator = registers.a as f64;
        let combo_value_option = get_combo_value(registers, instructions, pointer);
        if let Some(combo_value) = combo_value_option {
          let denominator = 2.0_f64.powi(combo_value as i32);
          let result = numerator / denominator;
          let str_result = result.to_string();
          let truncated_result = str_result.split(".").collect::<Vec<&str>>()[0]
            .parse::<i64>()
            .expect("failed to parse str_result to int");
          registers.c = truncated_result;
          *pointer += 2;
          return true;
        } else {
          return false;
        }
      }
      _ => {
        panic!("Invalid opcode");
      }
    }
  } else {
    return false;
  }
}

fn get_combo_value(
  registers: &mut Registers,
  instructions: &Vec<i64>,
  pointer: &mut i64,
) -> Option<i64> {
  let option = instructions.get(*pointer as usize + 1);
  if let Some(&val) = option {
    match val {
      0 => return Some(0),
      1 => return Some(1),
      2 => return Some(2),
      3 => return Some(3),
      4 => return Some(registers.a),
      5 => return Some(registers.b),
      6 => return Some(registers.c),
      7 => {
        panic!("Reserved operand")
      }
      _ => {
        unreachable!("Invalid operand")
      }
    }
  } else {
    return None;
  }
}

fn parse_input(input: &str) -> (Registers, Vec<i64>) {
  let split = input.split("\n\n").collect::<Vec<&str>>();

  let mut registers = Registers { a: 0, b: 0, c: 0 };
  let registers_split = split[0].split("\n").collect::<Vec<&str>>();
  registers.a = registers_split[0].split(": ").collect::<Vec<&str>>()[1]
    .parse::<i64>()
    .unwrap();
  registers.b = registers_split[1].split(": ").collect::<Vec<&str>>()[1]
    .parse::<i64>()
    .unwrap();

  registers.c = registers_split[2].split(": ").collect::<Vec<&str>>()[1]
    .parse::<i64>()
    .unwrap();

  let instr_str = split[1].split(": ").collect::<Vec<&str>>();

  let instructions = instr_str[1]
    .split(",")
    .collect::<Vec<&str>>()
    .to_vec()
    .iter()
    .map(|v| v.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();

  (registers, instructions)
}
