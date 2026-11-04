// tidal-rules: STG-18 STG-8 STG-14
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=type
// tidal-note: The int write fixes the set as one int. The read through unsigned is a read of that int, which STG-14 allows, so the type sanitizer reports nothing.
// tidal-note: GCC and Clang 18 have no type sanitizer and skip the test.
int main(int argc, char**) {
  void* raw = ::operator new(sizeof(int));
  *static_cast<int*>(raw) = argc;
  volatile unsigned seen = *static_cast<unsigned*>(raw);
  ::operator delete(raw);
  return seen == unsigned(argc) ? 0 : 1;
}
