// tidal-rules: RFOR-2
// tidal-expect: reject
struct R { int d[2]; int* begin() { return d; } };
int* end(R& r) { return r.d + 2; }
void f(R r) {
  for (int v : r) (void)v;
}
