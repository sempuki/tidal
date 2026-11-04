// tidal-rules: DEFARG-9 DEFARG-2
// tidal-expect: run
int f(int a = 1);
int f(int a) { return a; }
int g() {
  int f(int a = 2);   // another scope, so another default is valid
  return f();
}
int main() { return g() == 2 && f() == 1 ? 0 : 1; }
