// tidal-rules: IF-15
// tidal-expect: reject
void f() {
  if constexpr (false) {
    undeclared_function();
  }
}
