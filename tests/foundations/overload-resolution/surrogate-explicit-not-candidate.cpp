// tidal-rules: OVL-6
// tidal-expect: accept
// tidal-note: An explicit conversion function gives no surrogate, so the call operator is chosen although it needs a conversion.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
using Fn = One (*)(int);
struct Thing {
  explicit operator Fn() const;
  Two operator()(long) const;
};
Thing t;
static_assert(same<decltype(t(1)), Two>);
