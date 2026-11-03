// tidal-rules: CPY-15
// tidal-expect: accept
// tidal-note: Write report W7. The template would take its own class by value when K is N. A copy of the class uses the copy constructor, and the template serves only the other specializations.
template <int N> struct Money {
  int cents;
  constexpr Money(int c) : cents(c) {}
  template <int K> constexpr Money(Money<K> other) : cents(other.cents + 1000) {}
};
constexpr Money<1> a(5);
constexpr Money<1> b(a);
constexpr Money<1> c(Money<2>(5));
static_assert(b.cents == 5);
static_assert(c.cents == 1005);
