// tidal-rules: OVL-18 CLS-15
// tidal-expect: run
// tidal-note: Roadmap round 7, programmer reduction static_call_operator_address.cpp, template_function_address_explicit.cpp, and static_member_template_address_target.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). The address of a static member function template, with explicit arguments or from a target type, with or without &.
struct Sum {
  template <class T> static T add(T a, T b) { return a + b; }
  template <class T> static T operator()(T a, T b) { return a + b; }
};
int main() {
  auto f = &Sum::add<long>;
  auto g = &Sum::operator()<long>;
  long (*h)(long, long) = &Sum::add;
  long (*k)(long, long) = Sum::add;
  return f(2, 3) == 5 && g(2, 3) == 5 && h(2, 3) == 5 && k(2, 3) == 5 ? 0 : 1;
}
