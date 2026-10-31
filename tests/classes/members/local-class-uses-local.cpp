// tidal-rules: CLS-12
// tidal-expect: reject
// tidal-error-clang: reference to local variable .local. declared in enclosing function
// tidal-error-gcc: use of local variable with automatic storage from containing function
// tidal-error-edg: reference to local variable of enclosing function is not allowed
int f() {
  int local = 1;
  struct S { int get() { return local; } };
  return S{}.get();
}
