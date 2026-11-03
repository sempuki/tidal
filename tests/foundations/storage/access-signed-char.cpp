// tidal-rules: STG-23
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: signed char is not one of the types of STG-14 (Q-0306), so STG-23 makes the write undefined. The test checks only that the program compiles. No sanitizer in the matrix stops the run.
int store(int* number, signed char* other) { *number = 1; *other = 0; return *number; }
int main() {
  int (*volatile call)(int*, signed char*) = store;
  int value = 0;
  return call(&value, reinterpret_cast<signed char*>(&value));
}
