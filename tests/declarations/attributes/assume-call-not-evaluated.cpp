// tidal-rules: ATTR-15
// tidal-expect: run
// tidal-note: The assumption calls a function that changes a variable. The function is not called.
int calls = 0;
int bump() { return ++calls; }
int main() {
  [[assume(bump() > 0)]];
  return calls == 0 ? 0 : 1;
}
