// tidal-rules: ORD-9
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-error-gcc-14: is ambiguous
// tidal-note: The target is int(int&). Both templates give that type, read(T&) with T = int and read(T&&) with T = int&. The function types void(T&) and void(T&&) compare as one pair, and read(T&) is chosen. GCC 14 finds the address ambiguous.
template <class T> constexpr int read(T&) { return 1; }
template <class T> constexpr int read(T&&) { return 2; }
constexpr int (*p)(int&) = read;
static_assert(p == static_cast<int (*)(int&)>(read<int>));
