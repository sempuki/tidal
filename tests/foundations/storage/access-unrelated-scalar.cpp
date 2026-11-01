// tidal-rules: STG-23
// tidal-expect: run
// tidal-expect-gcc-16-o2: run-fail
// tidal-expect-clang-22-o2: run-fail
// tidal-note: At -O2, GCC and Clang assume that the float write does not change the int and return 1. STG-23 allows any result. Clang's type sanitizer reports the write.
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
