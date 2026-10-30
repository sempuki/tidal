// tidal-rules: UN-5
// tidal-expect: run
int one() { return 1; }
int main() {
  int (*f)() = one;
  return (*f)() == 1 && (**f)() == 1 ? 0 : 1;
}
