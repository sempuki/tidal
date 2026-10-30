// tidal-rules: STMT-2
// tidal-expect: reject
int f() {
  { int inner = 1; }
  return inner;
}
