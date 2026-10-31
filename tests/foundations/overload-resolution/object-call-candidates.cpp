// tidal-rules: OVL-5
// tidal-expect: accept
// tidal-note: The call operator and the surrogate are both candidates. Each call has only one viable candidate.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
using Pair = Two (*)(long, long);
struct Thing {
  One operator()(int) const;
  operator Pair() const;
};
Thing t;
static_assert(same<decltype(t(1)), One>);
static_assert(same<decltype(t(1, 2)), Two>);
