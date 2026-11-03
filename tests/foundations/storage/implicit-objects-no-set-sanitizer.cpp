// tidal-rules: STG-18
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=type
// tidal-error: read of size 4 .* with type float accesses an existing object of type int
// tidal-note: No set of objects lets the int and the float exist at once, so STG-18 makes the float read undefined.
// tidal-note: Run-time check with Clang's type sanitizer. The sanitizer reports the read and lets the program go on, and the runner counts the report as the failure. GCC and Clang 18 have no type sanitizer and skip the test.
int main(int argc, char**) {
  void* raw = ::operator new(sizeof(int) > sizeof(float) ? sizeof(int) : sizeof(float));
  *static_cast<int*>(raw) = argc;
  volatile float seen = *static_cast<float*>(raw);   // undefined
  (void)seen;
  ::operator delete(raw);
  return 0;
}
