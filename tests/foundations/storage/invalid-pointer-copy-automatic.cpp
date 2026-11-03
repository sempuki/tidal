// tidal-rules: STG-27
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: The storage of a block variable ends with its block. A copy of a pointer to the variable keeps the old address.
int main() {
  int* pointer;
  {
    int local = 1;
    pointer = &local;
  }
  int* copy = pointer;   // copies an invalid pointer value
  return copy == pointer ? 0 : 1;
}
