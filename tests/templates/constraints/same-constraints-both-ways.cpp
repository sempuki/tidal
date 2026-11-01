// tidal-rules: CNS-23 CNS-24
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The order of the operands does not matter, so each clause subsumes the other.
template <class T> concept A = sizeof(T) > 1;
template <class T> concept B = sizeof(T) < 100;
template <class T> struct Box {
  static constexpr int get() requires B<T> && A<T> { return 1; }
  static constexpr int get() requires A<T> && B<T> { return 2; }
};
int x = Box<int>::get();
