// tidal-rules: CMP-7
// tidal-expect: run
struct S { int a; int b; };
int main() {
  S s;
  return &s.a < &s.b ? 0 : 1;
}
