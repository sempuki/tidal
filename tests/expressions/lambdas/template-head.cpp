// tidal-rules: LAM-15
// tidal-expect: run
// tidal-note: Roadmap round 7, programmer reduction lambda_template_parameter_list.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). A lambda with a template head has a call operator template; a call deduces T, or names it.
int main() {
  auto twice = []<class T>(T x) { return x + x; };
  auto size_of = []<class T>() { return sizeof(T); };
  return twice(21) == 42 && twice(1.5) == 3.0 && size_of.operator()<char>() == 1 ? 0 : 1;
}
