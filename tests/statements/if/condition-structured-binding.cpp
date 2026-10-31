// tidal-rules: IF-6
// tidal-expect: reject
// tidal-error-clang: structured binding declaration in a condition
// tidal-error-gcc: structured bindings in conditions|expected unqualified-id before .\[
// tidal-error-edg: structured binding is not allowed in a condition
struct P { int a, b; explicit operator bool() const { return a; } };
void f() {
  if (auto [a, b] = P{1, 2}) {}
}
