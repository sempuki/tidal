// tidal-rules: OBJ-1 OBJ-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: not an integral constant expression
// tidal-note: Q-0368. set writes x through a pointer while the initializer of x runs. Clang rejects. GCC and EDG accept.
constexpr int set(int* p) { *p = 5; return 7; }
constexpr int make() { int x = set(&x); return x; }
static_assert(make() == 7);
