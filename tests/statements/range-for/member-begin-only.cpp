// tidal-rules: RFOR-7
// tidal-expect: reject
// tidal-error-clang: no viable .begin. function
// tidal-error-gcc: begin. was not declared in this scope
// tidal-error-edg: requires a suitable .begin. function
struct R { int d[2]; int* begin() { return d; } };
int* end(R& r) { return r.d + 2; }
void f(R r) {
  for (int v : r) (void)v;
}
