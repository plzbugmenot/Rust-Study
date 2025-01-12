struct Student {
  name: String,
  age: u32,
  grades: Vec<u32>,
}

impl Student {
  fn new(name: &str, age: u32, grades: Vec<u32>) -> Student {
      Student {
          name: String::from(name),
          age,
          grades,
      }
  }

  fn average_grade(&self) -> f64 {
      let sum: u32 = self.grades.iter().sum();
      let count = self.grades.len() as f64;
      if count == 0.0 {
          return 0.0;
      }
      sum as f64 / count
  }

  fn add_grade(&mut self, grade: u32) {
      self.grades.push(grade);
  }
}

fn main() {
  let mut student1 = Student::new("Alice", 20, vec![85, 90, 78]);
  assert_eq!(student1.name, "Alice");
  assert_eq!(student1.age, 20);
  assert_eq!(student1.grades, vec![85, 90, 78]);

  let student2 = Student::new("Bob", 22, vec![70, 80, 90]);
  assert_eq!(student2.name, "Bob");
  assert_eq!(student2.age, 22);
  assert_eq!(student2.grades, vec![70, 80, 90]);

  assert!((student1.average_grade() - 84.33333333333333).abs() < f64::EPSILON);
  assert!((student2.average_grade() - 80.0).abs() < f64::EPSILON);

  student1.add_grade(95);
  assert_eq!(student1.grades, vec![85, 90, 78, 95]);

  println!("All tests passed!");
}
