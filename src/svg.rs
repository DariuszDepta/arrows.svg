use std::fs;

const HTML_TEMPLATE: &str = include_str!("template.html");
const SVG_CONTENT: &str = "#SVG_CONTENT#";
const PI_2: f64 = std::f64::consts::PI * 2.0;

pub fn generate_html(file_name: &str) {
  let indent = 4;
  let mut svg = "".to_string();
  svg.push_str(&svg_begin(400.0, 400.0));

  svg.push_str(&svg_line_with_arrow(indent, 200.0, 200.0, 300.0, 300.0));
  svg.push_str(&svg_line_with_arrow(indent, 200.0, 200.0, 300.0, 230.0));
  svg.push_str(&svg_line_with_arrow(indent, 200.0, 200.0, 210.0, 300.0));

  svg.push_str(&svg_line_with_arrow(indent, 200.0, 200.0, 300.0, 100.0));
  svg.push_str(&svg_line_with_arrow(indent, 200.0, 200.0, 300.0, 190.0));
  svg.push_str(&svg_line_with_arrow(indent, 200.0, 200.0, 210.0, 10.0));

  svg.push_str(&svg_line_with_arrow(indent, 200.0, 200.0, 100.0, 300.0));
  svg.push_str(&svg_line_with_arrow(indent, 200.0, 200.0, 100.0, 210.0));

  svg.push_str(&svg_line_with_arrow(indent, 200.0, 200.0, 100.0, 100.0));
  svg.push_str(&svg_line_with_arrow(indent, 200.0, 200.0, 100.0, 10.0));

  svg.push_str(svg_end());
  fs::write(file_name, HTML_TEMPLATE.replace(SVG_CONTENT, &svg)).expect("writing HTML file failed");
}

/// Returns `<svg>` element.
fn svg_begin(width: f64, height: f64) -> String {
  format!(r#"<svg width="{}" height="{}">{}"#, width, height, '\n')
}

/// Returns `</svg>` element.
fn svg_end() -> &'static str {
  "</svg>\n"
}

fn svg_line(indent: usize, x1: f64, y1: f64, x2: f64, y2: f64) -> String {
  format!(r#"{:i$}<line x1="{}" y1="{}" x2="{}" y2="{}"/>{}"#, "", x1, y1, x2, y2, '\n', i = indent)
}

fn svg_arrow(indent: usize, x2: f64, y2: f64) -> String {
  format!(
    r#"{:i$}<path d="M {} {} l 10 -3 l 0 6 l -10 -3" fill="black" stroke-linecap="round"/>{}"#,
    "",
    x2,
    y2,
    '\n',
    i = indent
  )
}

/// Returns line.
fn svg_line_with_arrow(indent: usize, x1: f64, y1: f64, x2: f64, y2: f64) -> String {
  let mut content = "".to_string();
  content.push_str(&svg_line(indent, x1, y1, x2, y2));
  let a = rotation(x1, y1, x2, y2);
  content.push_str(&format!(r#"{:i$}<g transform="rotate({} {} {})">{}"#, "", a, x2, y2, '\n', i = indent));
  content.push_str(&svg_arrow(indent + 4, x2, y2));
  content.push_str(&format!(r#"{:i$}</g>{}"#, "", '\n', i = indent));
  content
}

fn rotation(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
  let x = x2 - x1;
  if x == 0.0 {
    return 0.0;
  }
  let y = y2 - y1;
  let angle = ((y / x).atan() * 360.0) / PI_2;
  if x > 0.0 {
    if y >= 0.0 {
      angle - 180.0
    } else {
      angle + 180.0
    }
  } else {
    angle
  }
}
