// tidal-rules: CONV-1 BF-6
// tidal-expect: accept
// tidal-note: Decision 0059, item RV5, point RV5-A-8. A bit-field of width 3 of an unscoped enumeration with the underlying type unsigned int promotes as the enumeration does, to unsigned int. A bit-field of integer type of the same width promotes to int.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
enum Level : unsigned { low, high };
struct Packed {
  Level level : 3;
  unsigned count : 3;
};
Packed p{};
static_assert(same<decltype(+p.level), unsigned>);
static_assert(same<decltype(+p.count), int>);
