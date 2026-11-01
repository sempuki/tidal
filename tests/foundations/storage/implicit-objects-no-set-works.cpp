// tidal-rules: STG-18
// tidal-expect: run
// tidal-note: No set of objects lets the int and the float exist at once. Clang's type sanitizer (-fsanitize=type, Clang 22) reports the float read; no other check in the matrix does.
int main(int argc, char**) {
  void* raw = ::operator new(sizeof(int) > sizeof(float) ? sizeof(int) : sizeof(float));
  *static_cast<int*>(raw) = argc;
  float seen = *static_cast<float*>(raw);   // undefined
  (void)seen;
  ::operator delete(raw);
  return 0;
}
