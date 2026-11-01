// tidal-rules: CNS-22 CNS-24
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Every type larger than 4 bytes is larger than 2 bytes, but subsumption compares atomic constraints only by identity.
template <class T> concept Over2 = sizeof(T) > 2;
template <class T> concept Over4 = sizeof(T) > 4;
template <class T> struct Box {
  static constexpr int get() requires Over2<T> { return 1; }
  static constexpr int get() requires Over4<T> { return 2; }
};
int x = Box<double>::get();
