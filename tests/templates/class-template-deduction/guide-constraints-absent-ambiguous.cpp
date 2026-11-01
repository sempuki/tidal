// tidal-rules: CTAD-29 CTAD-24
// tidal-expect: reject
// tidal-error-clang: ambiguous deduction for template arguments
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: more than one instance of constructor "Box" matches
// tidal-note: The class of guide-constraints.cpp without constraints. Both guides are viable for (1, 'a'), and neither is better, so the constraints decide in the constrained class.
template <class T> struct Box {
  template <class U> Box(U, T) {}
  template <class U> Box(T, U) {}
};
Box box(1, 'a');
