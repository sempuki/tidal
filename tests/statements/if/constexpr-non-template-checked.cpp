// tidal-rules: IF-23
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .undeclared_function
// tidal-error-gcc: undeclared_function. was not declared in this scope
// tidal-error-edg: identifier .undeclared_function. is undefined
void f() {
  if constexpr (false) {
    undeclared_function();
  }
}
