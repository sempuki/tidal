// tidal-rules: TPL-4
// tidal-expect: reject
// tidal-error-clang: template parameter list matching the non-templated nested type
// tidal-error-gcc: no declaration matches
// tidal-error-edg: template nesting depth does not match
// tidal-note: RV11-R-1. A friend declaration has two template heads, but the qualifier Shelf<int> does not use the parameter of the first head. Every compiler rejects it.
template <class T> struct Shelf {
  template <class U> int peek(U);
};
class Vault {
  template <class T> template <class U> friend int Shelf<int>::peek(U);
};
