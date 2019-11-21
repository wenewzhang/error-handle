#[derive(Debug)]
struct Earth {
  location: String,
}

#[derive(Debug)]
struct Dinosaur<'a> {
  location: &'a Earth,
  name: String,
}

fn main() {
  let new_york = Earth {
    location: "New York, NY".to_string(),
  };
  let t_rex = Dinosaur {
    location: &new_york,
    name: "T Rex".to_string(),
  };
  println!("{:?}", t_rex);
}
