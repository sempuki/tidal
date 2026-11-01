// tidal-rules: DEP-15 DEP-19
// tidal-expect: reject
// tidal-error-clang: expected ';' after expression
// tidal-error-gcc: need .typename. before .Box<M>::type.
// tidal-error-edg: use the "typename" keyword
template <int N> struct Box {
  static constexpr int M = N;
  using type = int;
  int get() { Box<M>::type local = 2; return local; }
};
int x = Box<3>{}.get();
