// tidal-rules: DEFARG-4
// tidal-expect: reject
// tidal-error-clang: default argument references local variable
// tidal-error-gcc: local variable .local. may not appear in this context
// tidal-error-edg: reference to local variable of enclosing function is not allowed
void g() {
  int local = 1;
  int f(int v = local);
}
