// tidal-rules: STG-27
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
int main() {
  int* pointer = new int(1);
  int* copy = pointer;
  delete pointer;
  int* again = copy;   // copies an invalid pointer value
  return again == copy ? 0 : 1;
}
