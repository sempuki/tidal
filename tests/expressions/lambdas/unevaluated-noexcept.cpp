// tidal-rules: LAM-14
// tidal-expect: accept
// tidal-note: A lambda in the operand of noexcept and in a default template argument.
static_assert(noexcept([] {}));
template <class T = decltype([](int x) { return x; })>
constexpr int apply(int v) { return T{}(v); }
static_assert(apply(3) == 3);
