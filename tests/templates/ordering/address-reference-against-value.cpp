// tidal-rules: ORD-9
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: is ambiguous
// tidal-note: The target is int(int&). Both templates give that type, read(T) with T = int& and read(T&) with T = int. The whole function types make one pair, and ORD-2 removes nothing inside them. read(T) is deduced from the invented void(X&), and read(T&) is not deduced from void(Y), so read(T&) is more specialized. Clang removes the reference and finds the address ambiguous (Q-0396).
template <class T> constexpr int read(T) { return 1; }
template <class T> constexpr int read(T&) { return 2; }
constexpr int (*p)(int&) = read;
static_assert(p == static_cast<int (*)(int&)>(read<int>));
