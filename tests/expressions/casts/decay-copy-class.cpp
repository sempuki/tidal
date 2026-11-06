// tidal-rules: CAST-13
// tidal-expect: run
// tidal-note: Roadmap round 7, programmer reduction auto_cast_class.cpp and auto_cast_class_in_template.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). auto(b) and auto{b} copy a class object.
struct Box { int v[2]{}; };
template <class T> T twice_copy(const T& t) { auto c = auto{t}; c.v[1] *= 2; return c; }
int main() {
  Box b{{3, 4}};
  auto copy = auto(b);
  copy.v[0] *= 2;
  return copy.v[0] == 6 && b.v[0] == 3 && twice_copy(b).v[1] == 8 && b.v[1] == 4 ? 0 : 1;
}
