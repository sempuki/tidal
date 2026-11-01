// tidal-rules: PFX-20
// tidal-expect: accept
// tidal-note: The member access can be in parentheses before the call. A static member through an object is an ordinary function.
struct C {
  int f(int v) { return v; }
  static int s(int v) { return v; }
} x;
int a = (x.f)(1);
auto b = x.s;
