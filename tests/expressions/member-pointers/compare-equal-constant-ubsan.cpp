// tidal-rules: MPTR-5
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: Machine::inc == Machine::neg.*is not a constant expression
// tidal-flags: -fsanitize=undefined
// tidal-note: Q-0333. Write report W9. GCC with the undefined-behavior sanitizer does not evaluate equality of member function pointers as a constant.
struct Machine {
  int inc(int v) const { return v + 1; }
  int neg(int v) const { return -v; }
};
constexpr int (Machine::*ops[2])(int) const = {&Machine::inc, &Machine::neg};
static_assert(ops[0] != ops[1]);
