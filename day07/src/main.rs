use rayon::prelude::*;

fn main() {
  let input = std::fs::read_to_string("./input.txt").expect("expected input data");
  let lines: Vec<Calibration> = input.lines().map(Calibration::from).collect();

  let p1: i64 = lines
    .par_iter()
    .map(|calibration| {
      if calibration.compute(false) {
        return calibration.solution;
      }
      return 0;
    })
    .sum();

  println!("part 1 = {}", p1);

  let p2: i64 = lines
    .par_iter()
    .map(|calibration| {
      if calibration.compute(true) {
        return calibration.solution;
      }
      return 0;
    })
    .sum();

  println!("part 2 = {}", p2);
}

#[derive(Debug)]
struct Calibration {
  solution: i64,
  equation: Vec<i64>,
}

impl Calibration {
  fn from(line: &str) -> Calibration {
    let components: Vec<&str> = line.split_whitespace().collect();
    let solution = components
      .get(0)
      .expect("expected first value")
      .trim_end_matches(":")
      .parse::<i64>()
      .expect("expected to parse first value");
    let equation: Vec<i64> = components[1..]
      .iter()
      .map(|x| x.parse::<i64>().expect("expected to parse all values"))
      .collect();
    return Calibration { solution, equation };
  }

  // compute addition
  fn compute(&self, use_concat: bool) -> bool {
    let mut equation_stack: Vec<Vec<i64>> = vec![self.equation.clone()];
    while equation_stack.len() > 0 {
      if let Some(equation) = equation_stack.pop() {
        match equation.len() {
          0 => panic!("an equation of length 0 shouldn't be possible"),
          // equation is fully resolved
          1 => {
            let value = equation
              .get(0)
              .expect("equation should have a single value");
            if *value == self.solution {
              return true;
            }
          }
          // equation has a lot of shit, combine first two els & concat the rest
          _ => {
            let a = equation
              .get(0)
              .expect("equation should have a single value");
            let b = equation
              .get(1)
              .expect("equation should have a single value");
            equation_stack.push([vec![a + b], equation[2..].to_vec()].concat());
            equation_stack.push([vec![a * b], equation[2..].to_vec()].concat());
            if use_concat {
              equation_stack
                .push([vec![concat(a.clone(), b.clone())], equation[2..].to_vec()].concat());
            }
          }
        }
      } else {
        break;
      }
    }
    return false;
  }
}

fn concat(a: i64, b: i64) -> i64 {
  return a * 10_i64.pow((f64::log10(b as f64) as u32) + 1) + b;
}
