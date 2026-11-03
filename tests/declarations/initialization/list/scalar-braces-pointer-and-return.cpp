// tidal-rules: LIST-1
// tidal-expect: accept
// tidal-note: A pointer from empty braces, a double from one value, and a return statement with braces.
constexpr int* p{};
constexpr double d = {2.5};
constexpr long make() { return {7}; }
static_assert(p == nullptr && d == 2.5 && make() == 7);
