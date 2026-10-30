// tidal-rules: REF-2
// tidal-expect: run
int make() { return 7; }
int main() {
  const int& a = 5;
  const int& b = make();
  return a == 5 && b == 7 ? 0 : 1;
}
