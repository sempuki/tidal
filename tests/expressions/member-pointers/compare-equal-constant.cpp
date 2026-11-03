// tidal-rules: MPTR-5 CE-1
// tidal-expect: accept
// tidal-note: Write report W9. Constant evaluation compares pointers to data members and to member functions.
struct Machine {
  int low, high;
  int inc(int v) const { return v + 1; }
  int neg(int v) const { return -v; }
};
constexpr int (Machine::*ops[2])(int) const = {&Machine::inc, &Machine::neg};
static_assert(ops[0] != ops[1]);
static_assert(ops[1] == &Machine::neg);
static_assert(&Machine::low != &Machine::high);
constexpr int Machine::*none = nullptr;
static_assert(none == nullptr);
