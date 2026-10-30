// tidal-rules: CLS-10
// tidal-expect: run
struct S { int a; int b; int c; };
int main() {
  S s{};
  return &s.a < &s.b && &s.b < &s.c ? 0 : 1;
}
