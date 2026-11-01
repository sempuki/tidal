// tidal-rules: STMT-8
// tidal-expect: accept
int shared = 4;
int f() {
  int g();
  int g();
  extern int shared;
  extern int shared;
  typedef int Count;
  typedef int Count;
  Count c = shared;
  return c;
}
