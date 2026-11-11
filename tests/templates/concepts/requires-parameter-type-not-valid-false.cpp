// tidal-rules: CPT-28
// tidal-expect: accept
// tidal-note: RV9-14. For int, the parameter type IteratorOf<int> is not valid, so each requires expression is false: in a concept, in a variable template, and in an if constexpr condition. For Count, the parameter type is valid, and each is true. Every compiler accepts.
template <class T> T&& value_of();
template <class R> using IteratorOf = decltype(value_of<R&>().begin());
struct Count {
  int* begin();
};
template <class R> concept Steps = requires(IteratorOf<R> it) { ++it; };
template <class R> constexpr bool steps = requires(IteratorOf<R> it) { ++it; };
template <class R> constexpr int pick() {
  if constexpr (requires(IteratorOf<R> it) { ++it; }) return 1;
  else return 2;
}
static_assert(!Steps<int> && Steps<Count>);
static_assert(!steps<int> && steps<Count>);
static_assert(pick<int>() == 2 && pick<Count>() == 1);
