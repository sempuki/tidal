// tidal-rules: ORD-9
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: is ambiguous
// tidal-note: The target is int(int&). read(T) gives it with T = int&, and read(T&&) with T = int&. The function types form one pair. read(T&&) is not deduced from the invented int(X), because X is not a reference. read(T) is deduced from int(Y&&) with T = Y&&. So read(T&&) is more specialized. Clang removes the references inside the function types and finds the address ambiguous (Q-0396).
template <class T> constexpr int read(T) { return 1; }
template <class T> constexpr int read(T&&) { return 2; }
constexpr int (*p)(int&) = read;
constexpr int call() {
  int n = 0;
  return p(n);
}
static_assert(call() == 2);
