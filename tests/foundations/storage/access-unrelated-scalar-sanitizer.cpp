// tidal-rules: STG-23
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=type
// tidal-error: write of size 4 .* with type float accesses an existing object of type int
// tidal-note: The object is an int, and STG-14 does not list float for it, so STG-23 makes the write undefined.
// tidal-note: Run-time check with Clang's type sanitizer. The sanitizer reports the write and lets the program go on, and the runner counts the report as the failure. GCC and Clang 18 have no type sanitizer and skip the test.
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
