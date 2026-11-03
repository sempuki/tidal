// tidal-rules: MPTR-5
// tidal-expect: accept
// tidal-flags: -fsanitize=undefined
// tidal-note: Write report W9. GCC with the undefined-behavior sanitizer evaluates equality of pointers to data members as a constant.
struct Machine { int low, high; };
constexpr int Machine::*fields[2] = {&Machine::low, &Machine::high};
static_assert(fields[0] != fields[1]);
