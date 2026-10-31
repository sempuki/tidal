// tidal-rules: SEQ-1 SEQ-8
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int counter = 0;
int bump() { int old = counter; counter = old + 1; return old; }
int main() {
  int sum = bump() + bump();
  return sum == 1 && counter == 2 ? 0 : 1;
}
