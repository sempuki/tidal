// tidal-rules: CPT-12
// tidal-expect: reject
// tidal-error-clang: trailing return type must appear before trailing requires clause
// tidal-error-gcc: base operand of .->. is not a pointer
// tidal-error-edg: cannot appear at the top level
// tidal-note: The member function of the class template puts its trailing requires clause before its trailing return type.
template <class T> struct Box {
  auto get() requires (sizeof(T) > 1) -> int;
};
