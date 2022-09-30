use crate::tuple::Tuple;

#[derive(Clone)]
pub struct Canvas{ width: usize, length: usize, matrix: Vec<Vec<Tuple>>}
impl Canvas{
  fn iter(&self) -> CanvasIter<'_> {
    CanvasIter{canvas: self, col: 0, row: 0}
  }
  pub fn new( width: usize, length: usize) -> Self {
    let row = vec![Tuple::color(0.0, 0.0, 0.0); width];
    let matrix = vec![row; length];
    Canvas{ width: width, length: length, matrix: matrix }
  }
  pub fn pixel_at(&self, x: usize, y: usize) -> Tuple {
    self.matrix[y][x]
  }
  pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
    self.matrix[y][x] = color;
  }

  pub fn fill_with(&mut self, color: Tuple) {
    for col in 0..self.width {
      for row in 0..self.length {
        self.write_pixel(col, row, color);
      }
    }
  }

  // generate a ppm file
  pub fn to_ppm(&self) -> String {
    PPM::parse(&self).lines
  }
}

struct ColorValue { val: String, len: usize }
  
  impl ColorValue {
    fn constrain(color: f64) -> i32 {
      const MAX: f64 = 255.00;
      const MIN: f64 = 0.0;
      let mut out = (color * MAX).ceil();
      if out > MAX {
        out = MAX;
      }
      if out < MIN {
        out = MIN
      }
      out as i32
    }
    pub fn new(color: f64) -> ColorValue {
      let color_string = Self::constrain(color).to_string();
      let color_length = color_string.chars().count();
      ColorValue{ val: color_string, len: color_length + 1}
    }
  }

struct PPM { lines: String}
impl PPM {
  pub fn parse(canvas: &Canvas) -> PPM {
    const LINE_LIMIT: usize = 66;
    fn whitespace(length: usize) -> &'static str {
      if length > LINE_LIMIT {
          "\n"
        }
        else {
          " " 
        }
    }
    fn new_length(ws: &str, line_length: usize, color_length: usize) -> usize {
      if ws == "\n" {
        0
      }
      else {
        line_length + color_length
      }
    }
    let mut lines = "P3\n".to_owned() + &canvas.width.to_string() + &" " + &canvas.length.to_string() + &"\n255\n";
    let mut line_length = 0;
    for p in canvas.iter() {
      
      // red
      let red = ColorValue::new(p.color.red());
      let mut ws: &str;
      ws = whitespace(line_length + red.len);
      lines.push_str(&(red.val + ws));
      line_length = new_length(ws, line_length, red.len);

      // green
      let green = ColorValue::new(p.color.green());
      ws = whitespace(line_length + green.len);
      lines.push_str(&(green.val + ws));
      line_length = new_length(ws, line_length, green.len);

      // blue
      let blue = ColorValue::new(p.color.blue());
      if line_length + blue.len > LINE_LIMIT || p.x == canvas.width - 1 {
        ws = "\n";
      }
      else {
        ws = " "; 
      }
      lines.push_str(&(blue.val + ws));
      line_length = new_length(ws, line_length, blue.len);
    }
    PPM{lines: lines}
  }
}

struct Pixel { x: usize, y: usize, color: Tuple }

struct CanvasIter<'a> { canvas: &'a Canvas, col: usize, row: usize }

impl Iterator for CanvasIter<'_> {
  type Item = Pixel;

  fn next(&mut self) -> Option<Self::Item> {
    if self.col == self.canvas.width {
      self.row += 1;
      self.col = 0;
    }
    if self.row == self.canvas.length {
      return None
    }
    let value = Some(Pixel{x: self.col, y: self.row, color: self.canvas.matrix[self.row][self.col]});
    self.col += 1;
    value
  }
}

#[test]
fn creating_a_canvas() {
  let c = Canvas::new(10, 20);
  assert_eq!(c.width, 10);
  assert_eq!(c.length, 20);
  // all the pixels are black
  for pixel in c.iter() { 
    assert!(pixel.color.equals(Tuple::color(0.0, 0.0, 0.0)));
  }
}
#[test]
fn writing_pixels_to_a_canvas() {
  let mut c = Canvas::new(10, 20);
  let red = Tuple::color(1.0, 0.0, 0.0);
  c.write_pixel(2, 3, red);
  assert!(c.pixel_at(2,3).equals(red));
}
#[test]
fn constructing_the_ppm_header() {
  let c = Canvas::new(5, 3);
  let ppm = c.to_ppm();
  let lines: Vec<&str> = ppm.split("\n").collect();
  assert_eq!(lines[0], "P3");
  assert_eq!(lines[1], "5 3");
  assert_eq!(lines[2], "255");
}
#[test]
fn constructing_the_ppm_pixel_data() {
  let mut c = Canvas::new(5, 3);
  let c1 = Tuple::color(1.5, 0.0, 0.0);
  let c2 = Tuple::color(0.0, 0.5, 0.0);
  let c3 = Tuple::color(-0.5, 0.0, 1.0);
  c.write_pixel(0, 0, c1);
  c.write_pixel(2, 1, c2);
  c.write_pixel(4, 2, c3);
  let ppm = c.to_ppm();
  let lines: Vec<&str> = ppm.split("\n").collect();
  assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
  assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
  assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
}
#[test]
fn splitting_long_lines_in_ppm_files() {
  let mut c = Canvas::new(10, 2);
  let c1 = Tuple::color(1.0, 0.8, 0.6);
  c.fill_with(c1);
  let ppm = c.to_ppm();
  let lines: Vec<&str> = ppm.split("\n").collect();
  
  assert_eq!(lines[3], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
  assert_eq!(lines[4], "153 255 204 153 255 204 153 255 204 153 255 204 153");
  assert_eq!(lines[5], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
  assert_eq!(lines[6], "153 255 204 153 255 204 153 255 204 153 255 204 153");
}
#[test]
fn ppm_files_are_terminated_by_a_newline_character() {
  let c = Canvas::new(5, 3);
  let ppm = c.to_ppm();
  assert_eq!(&ppm[ppm.len() - 1..ppm.len()], "\n");
}
