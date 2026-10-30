// tidal-rules: CMP-6
// tidal-expect: accept
bool f(int* p, const int* q) {
  return p == q && !(p < q);
}
