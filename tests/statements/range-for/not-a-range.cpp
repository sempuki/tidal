// tidal-rules: RFOR-7
// tidal-expect: reject
// tidal-error-clang: no viable .begin. function
// tidal-error-gcc: begin. was not declared in this scope
// tidal-error-edg: requires a suitable .begin. function
void f(int* p) {
  for (int v : p) (void)v;
}
