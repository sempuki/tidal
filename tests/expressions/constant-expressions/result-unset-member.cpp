// tidal-rules: CE-21
// tidal-expect: reject
// tidal-error-clang: subobject .second. is not initialized
// tidal-error-gcc: incompletely initialized variable
// tidal-error-edg: access to uninitialized subobject
struct Pair {
  int first;
  int second;
  constexpr Pair() : first(1) {}
};
constinit Pair pair;
