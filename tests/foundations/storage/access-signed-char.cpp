// tidal-rules: STG-23
// tidal-expect: run
// tidal-note: signed char is not one of the types of STG-14 (Q-0306). Every compiler in the matrix lets the write change the int, at -O0 and -O2.
int store(int* number, signed char* other) { *number = 1; *other = 0; return *number; }
int main() {
  int (*volatile call)(int*, signed char*) = store;
  int value = 0;
  return call(&value, reinterpret_cast<signed char*>(&value));
}
