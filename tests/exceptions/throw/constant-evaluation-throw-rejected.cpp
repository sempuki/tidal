// tidal-rules: CE-8
// tidal-expect: reject
// tidal-error-gcc: throw-expression.. is not a constant expression
constexpr int check(int x) { if (x < 0) throw 1; return x; }
static_assert(check(-1) == 0);
