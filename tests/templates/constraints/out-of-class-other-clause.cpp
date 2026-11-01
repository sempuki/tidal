// tidal-rules: CNS-29
// tidal-expect: reject
// tidal-error-clang: does not match any declaration
// tidal-error-gcc: no declaration matches
// tidal-error-edg: declaration is incompatible
// tidal-note: The definition outside the class has a requires clause that is not equivalent to the member's clause.
template <class T> struct Box {
  int get() requires (sizeof(T) > 1);
};
template <class T> int Box<T>::get() requires (sizeof(T) > 2) { return 1; }
