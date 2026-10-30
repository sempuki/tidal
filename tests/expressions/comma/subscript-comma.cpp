// tidal-rules: COMMA-6
// tidal-expect: reject
int f(int* p, int i, int j) {
  return p[i, j];
}
