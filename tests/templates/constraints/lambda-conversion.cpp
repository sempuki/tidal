// tidal-rules: CNS-18
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: EDG rejects the trailing clause on the lambda (Q-0260).
template <class T> constexpr auto make() {
  return [](int x) requires (sizeof(T) > 1) { return x; };
}
template <class L> concept ToPointer = requires (L lam) { static_cast<int (*)(int)>(lam); };
static_assert(ToPointer<decltype(make<int>())> && !ToPointer<decltype(make<char>())>);
