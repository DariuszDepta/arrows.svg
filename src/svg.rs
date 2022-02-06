use std::fs;

const HTML_TEMPLATE: &str = include_str!("template.html");
const SVG_CONTENT: &str = "#SVG_CONTENT#";
const PI_2: f64 = std::f64::consts::PI * 2.0;

pub fn generate_html(file_name: &str) {
  let mut svg = "".to_string();
  svg.push_str(&svg_begin(300.0, 300.0));

  svg.push_str(&svg_line_with_arrow(0.0, 0.0, 100.0, 100.0));

  svg.push_str(&svg_line_with_arrow(10.0, 200.0, 100.0, 10.0));

  svg.push_str(svg_end());
  fs::write(file_name, HTML_TEMPLATE.replace(SVG_CONTENT, &svg)).expect("writing HTML file failed");
}

/// Returns `<svg>` element.
fn svg_begin(width: f64, height: f64) -> String {
  format!("<svg width=\"{}\" height=\"{}\">\n", width, height)
}

/// Returns `</svg>` element.
fn svg_end() -> &'static str {
  "</svg>\n"
}

/// Returns line.
fn svg_line_with_arrow(x1: f64, y1: f64, x2: f64, y2: f64) -> String {
  let mut content = "".to_string();
  content.push_str(&format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n", x1, y1, x2, y2));
  let a = angle(x1, y1, x2, y2);
  content.push_str(&format!(r#"<g transform="rotate({} {} {})">"#, a, x2, y2));
  content.push_str(&format!(
    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" style="stroke:rgb(255,0,0)"/>"#,
    x2,
    y2,
    x2 + 10.0,
    y2
  ));
  content.push_str("</g>\n");
  content
}

fn angle(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
  let ex = x2 - x1;
  let ey = y2 - y1;
  let mut angle = (ex / ey).atan();
  -(180.0 - ((angle * 360.0) / PI_2))
}
