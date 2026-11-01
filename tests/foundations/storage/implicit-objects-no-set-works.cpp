// tidal-rules: STG-18
// tidal-expect: accept
// tidal-note: No set of objects lets the int and the float exist at once, so STG-18 makes the float read undefined. The test checks only that the program compiles. Only the type sanitizer of Clang 22 (-fsanitize=type) reports the access, and the run goes on with exit status 0.
int main(int argc, char**) {
  void* raw = ::operator new(sizeof(int) > sizeof(float) ? sizeof(int) : sizeof(float));
  *static_cast<int*>(raw) = argc;
  float seen = *static_cast<float*>(raw);   // undefined
  (void)seen;
  ::operator delete(raw);
  return 0;
}
