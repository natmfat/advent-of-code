fn main() {
  let mut language = Language::new(
    std::fs::read_to_string("./input.txt")
      .expect("expected input data")
      .to_string(),
  );

  println!("part 1 = {}", language.core(true));

  language.reset();
  println!("part 2 = {}", language.core(false));
}

struct Language {
  pos: usize,
  tokens: Vec<char>,
}

impl Language {
  fn new(raw: String) -> Language {
    let tokens = raw.chars().collect::<Vec<char>>();
    return Language { pos: 0, tokens };
  }

  fn reset(&mut self) {
    self.pos = 0;
  }

  fn get(&self, idx: usize) -> Option<char> {
    return self.tokens.get(idx).copied();
  }

  fn curr(&self) -> Option<char> {
    return self.get(self.pos);
  }

  fn next(&mut self) {
    self.pos += 1;
  }

  /// does this set of tokens match the provided sequence
  /// otherwise, reset back to initial pos
  fn matches(&mut self, sequence: Vec<char>) -> bool {
    let initial_pos = self.pos;

    for i in 0..sequence.len() {
      let curr = sequence.get(i).expect("expected to get sequence at i");
      if self.get(i + self.pos) != Some(*curr) {
        // restore initial pos
        self.pos = initial_pos;
        return false;
      }
    }

    self.pos += sequence.len();
    return true;
  }

  fn extract_numeric(&mut self) -> i32 {
    let mut number: String = String::new();
    while let Some(curr) = self.curr() {
      if char::is_numeric(curr) {
        number.push(curr);
        self.next();
      } else {
        break;
      }
    }
    return number.parse::<i32>().expect("expected to parse number");
  }

  fn core(&mut self, always_enabled: bool) -> i32 {
    let mut result = 0;
    let mut enabled = true;

    while self.pos < self.tokens.len() {
      if self.matches(vec!['d', 'o', 'n', '\'', 't', '(', ')']) {
        enabled = false;
      } else if self.matches(vec!['d', 'o', '(', ')']) {
        enabled = true;
      } else if self.matches(vec!['m', 'u', 'l', '(']) {
        let a = self.extract_numeric();
        if self.matches(vec![',']) {
          let b = self.extract_numeric();
          if self.matches(vec![')']) && (enabled || always_enabled) {
            result += a * b;
          }
        }
      } else {
        self.next();
      }
    }
    return result;
  }
}
