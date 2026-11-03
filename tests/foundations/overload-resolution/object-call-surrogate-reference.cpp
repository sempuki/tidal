// tidal-rules: OVL-5
// tidal-expect: accept
// tidal-note: The surrogate comes from a conversion to a reference to a function. Each call has only one viable candidate.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
using Ref = Two (&)(const char*);
struct Thing {
  One operator()(int, int) const;
  operator Ref() const;
};
Thing t;
static_assert(same<decltype(t(1, 2)), One>);
static_assert(same<decltype(t("s")), Two>);
