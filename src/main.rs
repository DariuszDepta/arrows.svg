use crate::svg::generate_html;
use std::f64::consts::PI;

mod svg;

fn main() {
  let angle = get_angle((0.0, 0.0), (0.0, 1.0));
  println!("radians = {}", angle);
  println!("degrees = {}", radians_to_degrees(angle));
  generate_html("./output/index.html");
}

fn get_angle(begin: (f64, f64), end: (f64, f64)) -> f64 {
  let dx = -begin.0;
  let dy = -begin.1;
  let ex = end.0 + dx;
  let ey = end.1 + dy;
  (ex / ey).atan()
}

fn radians_to_degrees(r: f64) -> f64 {
  (r * 360.0) / (2.0 * PI)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f64::consts::FRAC_PI_4;

  #[test]
  fn test_angle() {
    assert_eq!(FRAC_PI_4, get_angle((0.0, 0.0), (1.0, 1.0)));
    assert_eq!(-FRAC_PI_4, get_angle((0.0, 0.0), (-1.0, 1.0)));
    assert_eq!(FRAC_PI_4, get_angle((0.0, 0.0), (-1.0, -1.0)));
    assert_eq!(-FRAC_PI_4, get_angle((0.0, 0.0), (1.0, -1.0)));
  }

  #[test]
  fn test_degree() {
    assert_eq!(45.0, radians_to_degrees(get_angle((0.0, 0.0), (1.0, 1.0))));
    assert_eq!(-45.0, radians_to_degrees(get_angle((0.0, 0.0), (-1.0, 1.0))));
    assert_eq!(45.0, radians_to_degrees(get_angle((0.0, 0.0), (-1.0, -1.0))));
    assert_eq!(-45.0, radians_to_degrees(get_angle((0.0, 0.0), (1.0, -1.0))));
  }
}
