// tidal-rules: RFOR-2
// tidal-expect: reject
void f(int* p) {
  for (int v : p) (void)v;
}
