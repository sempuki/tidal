// tidal-rules: DEFARG-4
// tidal-expect: accept
void g() {
  int local = 1;
  int f(int v = sizeof(local));
  (void)local;
}
