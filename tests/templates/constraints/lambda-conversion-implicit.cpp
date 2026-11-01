// tidal-rules: CNS-18
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: The conversion to a pointer to function happens in an initialization. For make<int> the constraint holds. EDG rejects the trailing clause on the lambda (Q-0260).
template <class T> constexpr auto make() {
  return [](int x) requires (sizeof(T) > 1) { return x + 1; };
}
template <class L> concept Converts = requires (L lam) { [](int (*)(int)) {}(lam); };
int (*inc)(int) = make<int>();
static_assert(Converts<decltype(make<long>())> && !Converts<decltype(make<char>())>);
