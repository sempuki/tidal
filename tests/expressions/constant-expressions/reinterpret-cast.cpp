// tidal-rules: CE-4
// tidal-expect: reject
// tidal-error-gcc: reinterpret_cast. is not a constant expression
// tidal-error-clang: reinterpret_cast is not allowed in a constant expression
// tidal-error-edg: is invalid in constant-expression evaluation
constexpr int f() {
  int x = 1;
  return *reinterpret_cast<char*>(&x);
}
static_assert(f() == 1);
