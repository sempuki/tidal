// tidal-rules: STG-23
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: STG-23 makes the float write undefined, so the test checks only that the program compiles. Only the type sanitizer of Clang 22 (-fsanitize=type) reports the access, and the run goes on with exit status 0.
int store(int* number, float* real) {
  *number = 1;
  *real = 0.0f;   // undefined: the object is an int
  return *number;
}
int main() {
  int (*volatile call)(int*, float*) = store;
  int value = 0;
  return call(&value, reinterpret_cast<float*>(&value));
}
