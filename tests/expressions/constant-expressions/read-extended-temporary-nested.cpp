// tidal-rules: CE-3 CE-14
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: temporary.* is not usable in a constant expression
// tidal-expect-edg: reject
// tidal-error-edg: temporary with limited
// tidal-note: Q-0193
struct Holder { const int& n; };
constexpr const Holder& held = Holder{4};
static_assert(held.n == 4);
