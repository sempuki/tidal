// tidal-rules: VC-4
// tidal-expect: run
int made = 0;
struct S { S() { ++made; } };
S make() { return S(); }
int main() {
  S a = S(S(S()));
  S b = make();
  return made == 2 ? 0 : 1;
}
