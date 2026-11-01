// tidal-rules: TSP-21
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: static assertion failed
// tidal-note: The primary member template is given explicitly for Outer<short>, so its pointer partial specialization is not used there. Clang 18 selects the partial specialization (Q-0206).
template <class T> struct Outer {
  template <class U> struct Inner { static constexpr int kind = 0; };
  template <class U> struct Inner<U*> { static constexpr int kind = 1; };
};
template <> template <class U> struct Outer<short>::Inner { static constexpr int kind = 2; };
static_assert(Outer<short>::Inner<int*>::kind == 2);
static_assert(Outer<int>::Inner<int*>::kind == 1);
