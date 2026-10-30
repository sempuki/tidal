// tidal-rules: CLS-6
// tidal-expect: run
struct S {
  int which() & { return 1; }
  int which() && { return 2; }
};
int main() {
  S s;
  return s.which() == 1 && S{}.which() == 2 ? 0 : 1;
}
