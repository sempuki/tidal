// tidal-rules: CE-26
// tidal-expect: accept
// The values of these constants are undefined. The test checks only that
// the implementation can accept them. The pairs are two assignments, and
// a read with an increment, as operands of + and -.
constexpr int two_assignments() { int i = 0; return (i = 1) + (i = 2); }
constexpr int read_and_increment() { int i = 1; return i - ++i; }
constexpr int a = two_assignments();
constexpr int b = read_and_increment();
