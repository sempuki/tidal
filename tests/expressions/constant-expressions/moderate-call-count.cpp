// tidal-rules: CE-10
// tidal-expect: accept
// tidal-note: Write report W5. fib(24) makes 150049 calls with a depth of only 24.
constexpr long fib(int n) { return n < 2 ? n : fib(n - 1) + fib(n - 2); }
static_assert(fib(24) == 46368);
