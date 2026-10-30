// tidal-rules: CINIT-2
// tidal-expect: accept
void f() {
  static constinit int x = 1;
  thread_local constinit int y = 2;
}
