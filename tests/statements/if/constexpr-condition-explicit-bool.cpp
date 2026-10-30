// tidal-rules: IF-16
// tidal-expect: accept
struct S { constexpr explicit operator bool() const { return true; } };
void f() {
  if constexpr (S{}) {}
}
