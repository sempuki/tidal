// tidal-rules: OBJ-1 OBJ-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: not an integral constant expression
// tidal-note: Q-0368. The lifetime of x begins when its initializer completes, so the assignment inside the initializer writes x outside its lifetime. Clang rejects. GCC and EDG accept.
constexpr int make() { int x = ((x = 5), x + 1); return x; }
static_assert(make() == 6);
