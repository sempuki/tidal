// tidal-rules: OPR-3 DED-1
// tidal-expect: run
// tidal-note: Roadmap round 7, programmer reduction call_operator_template_run.cpp and call_operator_template_class_param.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). A call of an object deduces the arguments of a member operator() template.
struct Id { template <class T> T operator()(T a) const { return a; } };
struct Box { int v; };
int main() {
  Id id;
  return Id{}(3) == 3 && id(Box{4}).v == 4 && id(2.5) == 2.5 ? 0 : 1;
}
