// tidal-rules: STO-4
// tidal-expect: accept
thread_local int a = 1;
static thread_local int b = 2;
extern thread_local int c;
thread_local int c = 3;
void f() {
  static thread_local int d = 4;
  thread_local int e = 5;
  (void)d;
  (void)e;
}
