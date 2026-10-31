// tidal-rules: OVL-24
// tidal-expect: accept
// tidal-note: The member with constraints that the class does not satisfy is not viable, so the other member wins although its sequence is worse.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
template <class T> concept Wide = sizeof(T) >= 8;
template <class T> struct Box {
  One put(T) requires Wide<T>;
  Two put(long double);
};
static_assert(same<decltype(Box<double>{}.put(1.0)), One>);
static_assert(same<decltype(Box<char>{}.put('a')), Two>);
