// tidal-rules: CNS-4 CNS-24
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The two (!A<T>) are separate atomic constraints. named-negation.cpp moves the negation into a concept.
template <class T> concept A = sizeof(T) > 1;
template <class T> struct Box {
  static constexpr int get() requires (!A<T>) { return 1; }
  static constexpr int get() requires (!A<T>) && (sizeof(T) > 0) { return 2; }
};
int x = Box<char>::get();
