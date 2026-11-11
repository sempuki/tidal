// tidal-rules: CTOR-10 ICS-9 DED-1
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer reduction closure_argument_converting_constructor_template.cpp (from the tries of fnref.cpp). Callback has a constructor template that is not explicit, so the specialization for a closure type is a converting constructor (CTOR-10). A closure argument of run initializes the parameter of type Callback through one user-defined conversion (ICS-9), and deduction gives F the closure type (DED-1). The call works with a closure with no capture and with one that captures by copy.
struct Callback {
  int value;
  template <class F> constexpr Callback(F f) : value(f()) {}
};
constexpr int run(Callback c) { return c.value; }
static_assert(run([] { return 3; }) == 3);
int main() {
  int base = 4;
  return run([] { return 3; }) + run([base] { return base * 2; }) == 11 ? 0 : 1;
}
