// tidal-rules: TARG-27
// tidal-expect: reject
// tidal-error-clang: found in multiple base classes of different types
// tidal-error-gcc: template argument 1 is invalid
// tidal-error-edg: is ambiguous
// tidal-note: Base as a template template argument is ambiguous too.
template <class T> struct Base {};
struct Both : Base<int>, Base<char> {
  template <template <class> class C> struct Use {};
  Use<Base> u;
};
