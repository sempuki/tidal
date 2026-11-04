// tidal-rules: CE-10
// tidal-expect: reject
// tidal-error-gcc: operation count exceeds limit
// tidal-error-clang: maximum step limit
// tidal-error-edg: excessive constexpr function call complexity
// tidal-note: Write report W5. fib(40) makes about 331 million calls with a depth of only 40. The total work exceeds every matrix limit.
constexpr long fib(int n) { return n < 2 ? n : fib(n - 1) + fib(n - 2); }
static_assert(fib(40) == 102334155);
