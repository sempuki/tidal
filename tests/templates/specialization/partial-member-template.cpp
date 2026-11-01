// tidal-rules: TSP-9
// tidal-expect: accept
// tidal-note: Partial specializations of member templates, one in the class and one outside with both heads.
template <class T> struct Outer {
  template <class U> struct Inner { static constexpr int kind = 0; };
  template <class U> struct Inner<U*> { static constexpr int kind = 1; };
  template <class U> struct Other { static constexpr int kind = 0; };
};
template <class T> template <class U> struct Outer<T>::Other<U&> { static constexpr int kind = 2; };
static_assert(Outer<int>::Inner<char*>::kind == 1 && Outer<int>::Inner<char>::kind == 0);
static_assert(Outer<int>::Other<char&>::kind == 2 && Outer<int>::Other<char>::kind == 0);
