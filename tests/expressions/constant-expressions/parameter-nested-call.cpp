// tidal-rules: CE-30 CE-3
// tidal-expect: accept
// tidal-note: A call inside the body is part of the running evaluation, so the parameter is usable as its argument.
consteval int twice(int n) { return n * 2; }
consteval int next_twice(int n) { return twice(n) + 1; }
static_assert(next_twice(3) == 7);
