// tidal-rules: STG-18 STG-8 UNI-5
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=type
// tidal-note: The storage holds a Cell. The assignment to real makes real the active member, so the later read of real is valid after the write of whole. The type sanitizer reports nothing.
// tidal-note: GCC and Clang 18 have no type sanitizer and skip the test.
union Cell { int whole; float real; };
int main(int argc, char**) {
  void* raw = ::operator new(sizeof(Cell));
  Cell* cell = static_cast<Cell*>(raw);
  cell->whole = argc;
  cell->real = 2.0f;
  volatile float seen = cell->real;
  ::operator delete(raw);
  return seen == 2.0f ? 0 : 1;
}
