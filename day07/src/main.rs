fn main() {
  let input = std::fs::read_to_string("./input.txt").expect("expected input data");
  let lines: Vec<Calibration> = input.lines().map(Calibration::from).collect();

  for calibration in lines {
    println!("{:?}", Calibration::compute(calibration.equation));
  }
}

#[derive(Debug)]
struct Calibration {
  expected: i32,
  equation: Vec<i32>,
}

impl Calibration {
  fn from(line: &str) -> Calibration {
    let components: Vec<&str> = line.split_whitespace().collect();
    let expected = components
      .get(0)
      .expect("expected first value")
      .trim_end_matches(":")
      .parse::<i32>()
      .expect("expected to parse first value");
    let equation: Vec<i32> = components[1..]
      .iter()
      .map(|x| x.parse::<i32>().expect("expected to parse all values"))
      .collect();
    return Calibration { expected, equation };
  }

  // compute addition
  fn compute(&self) -> i32 {
    return 0;
    // let len = self.equation.len();
    // if len == 1 {
    //   let a = equation.get(0).expect("expected first value").clone();
    //   return a;
    // }
    // if len == 2 {
    //   let a = equation.get(0).expect("expected first value").clone();
    //   let b = equation.get(1).expect("expected second value").clone();
    //   return a + b;
    // } else {
    //   return Calibration::compute(equation[0..2].to_vec())
    //     + Calibration::compute(equation[2..].to_vec());
    // }
  }
}
