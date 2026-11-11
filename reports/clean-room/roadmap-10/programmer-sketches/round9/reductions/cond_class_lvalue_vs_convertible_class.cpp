// The conditional operator: a class lvalue against an lvalue of another
// class that converts to it through a converting constructor (COND-16).
struct Tag { constexpr explicit Tag(int) {} };
struct Box {
  int v = 0;
  constexpr Box(int x) : v(x) {}
  constexpr Box(Tag) {}
};
int main(int argc, char**) {
  Box b = 7;
  const Tag t{0};
  Box r = argc > 0 ? b : t;
  return r.v == 7 ? 0 : 1;
}
