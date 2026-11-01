// tidal-rules: DEP-15 DEP-19
// tidal-expect: reject
// tidal-error-clang: expected ';' after expression
// tidal-error-gcc: need .typename. before .Box<T\*>::type.
// tidal-error-edg: use the "typename" keyword
template <class T> struct Box {
  using type = int;
  int get() { Box<T*>::type a = 0; return a; }
};
