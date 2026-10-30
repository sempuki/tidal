// tidal-rules: DEFARG-4
// tidal-expect: reject
void g() {
  int local = 1;
  int f(int v = local);
}
