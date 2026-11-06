// Shapes behind a trait object, with a default method and dynamic dispatch.
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str { "shape" }
}
struct Circle { r: f64 }
struct Rect { w: f64, h: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { 3.0 * self.r * self.r }
    fn name(&self) -> &'static str { "circle" }
}
impl Shape for Rect {
    fn area(&self) -> f64 { self.w * self.h }
}
fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle { r: 1.0 }), Box::new(Rect { w: 2.0, h: 3.0 })];
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    let named = shapes.iter().filter(|s| s.name() != "shape").count();
    std::process::exit(if total == 9.0 && named == 1 { 0 } else { 1 });
}
