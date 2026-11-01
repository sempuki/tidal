// tidal-rules: TSP-26
// tidal-expect: reject
// tidal-error-clang: template specialization requires 'template<>'
// tidal-error-gcc: requires .template<>. syntax
// tidal-error-edg: template nesting depth does not match
// tidal-note: Two class templates are fixed, but the declaration has one template <>.
template <class T> struct Outer {
  template <class U> struct Inner { int get() { return 0; } };
};
template <> int Outer<char>::Inner<int>::get() { return 1; }
