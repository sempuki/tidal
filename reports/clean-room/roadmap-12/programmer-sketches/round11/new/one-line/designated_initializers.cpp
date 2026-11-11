// Designated initializers with a default member initializer left out.
struct Options { int width = 80; int height = 24; bool wrap = false; };
int area(Options o) { return o.width * o.height + o.wrap; }
int main() {
  return area({.width = 10, .wrap = true}) == 241 ? 0 : 1;
}
