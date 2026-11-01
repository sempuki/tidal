// tidal-rules: PACK-26 PACK-6
// tidal-expect: reject
// tidal-error-clang: too few template arguments
// tidal-error-gcc: wrong number of template arguments
// tidal-error-edg: too few arguments for class template
// tidal-note: W expands the outer pack T of two elements, so Shaped takes two templates, and one is given.
template <class... T> struct Outer {
  template <template <T> class... W> struct Shaped {};
};
template <int> struct ByInt {};
Outer<int, char>::Shaped<ByInt> bad;
