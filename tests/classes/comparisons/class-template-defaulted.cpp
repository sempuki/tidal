// tidal-rules: DCMP-3
// tidal-expect: accept
// tidal-note: Write report W7. A defaulted comparison declared in a class template is not a function template. Each specialization gets its own comparison.
template <int Currency> struct Money {
  long cents;
  friend bool operator==(const Money&, const Money&) = default;
  bool operator!=(const Money&) const = default;
};
static_assert(Money<1>{5} == Money<1>{5});
static_assert(Money<2>{5} != Money<2>{6});
template <class T> concept equal_comparable = requires(T a, T b) { a == b; };
template <class A, class B> concept cross_comparable = requires(A a, B b) { a == b; };
static_assert(equal_comparable<Money<1>>);
static_assert(!cross_comparable<Money<1>, Money<2>>);
