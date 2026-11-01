// tidal-rules: STG-29 STG-23
// tidal-expect: accept
// tidal-note: Q-0324. The cast gives an int* that still points to the first byte of buffer (STG-16), so the write through it reaches an unsigned char object through an int glvalue (STG-29, STG-23). The provisional answer makes the program undefined. No configuration in the matrix stops the program, so the test checks only that it compiles.
int main() {
  alignas(int) unsigned char buffer[sizeof(int)];
  int* number = reinterpret_cast<int*>(buffer);
  *number = 5;                         // undefined: no int is reached
  return *number == 5 ? 0 : 1;
}
