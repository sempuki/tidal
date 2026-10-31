// tidal-rules: UNI-7
// tidal-expect: run
int main() {
  union { int i; float f; };
  i = 4;
  return i == 4 ? 0 : 1;
}
