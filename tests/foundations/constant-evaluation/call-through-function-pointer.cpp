// tidal-rules: CE-1
// tidal-expect: accept
// tidal-note: Write report W5. A call through a pointer to a constexpr function, from a table and from a parameter.
constexpr int twice(int v) { return 2 * v; }
constexpr int negate(int v) { return -v; }
using Handler = int (*)(int);
constexpr Handler handlers[2] = {twice, negate};
constexpr int apply(Handler h, int v) { return h(v); }
static_assert(handlers[0](4) == 8);
static_assert(apply(handlers[1], 3) == -3);
