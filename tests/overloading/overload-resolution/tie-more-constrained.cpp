// tidal-rules: OVL-30
// tidal-expect: accept
// tidal-note: The two members are plain functions with the same parameters. Both are viable for Box<int>, and the second is more constrained. Neither is a template, so OVL-27 and OVL-28 do not apply.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
template <class T> concept Small = sizeof(T) <= 8;
template <class T> concept SmallWhole = Small<T> && (T(1) / 2 == 0);
template <class T> struct Box {
  One get() requires Small<T>;
  Two get() requires SmallWhole<T>;
};
static_assert(same<decltype(Box<int>{}.get()), Two>);
static_assert(same<decltype(Box<double>{}.get()), One>);
