// tidal-rules: TSP-21
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: One partial specialization is given explicitly for Outer<short>. The primary and the other partial specialization still apply. GCC does not select the explicit one (Q-0207).
template <class T> struct Outer {
  template <class U> struct Inner { static constexpr int kind = 0; };
  template <class U> struct Inner<U*> { static constexpr int kind = 1; };
  template <class U> struct Inner<U&> { static constexpr int kind = 3; };
};
template <> template <class U> struct Outer<short>::Inner<U*> { static constexpr int kind = 2; };
static_assert(Outer<short>::Inner<int*>::kind == 2);
static_assert(Outer<short>::Inner<int&>::kind == 3);
static_assert(Outer<short>::Inner<int>::kind == 0);
