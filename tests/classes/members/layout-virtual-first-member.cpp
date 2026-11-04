// tidal-rules: CLS-34 CLS-35
// tidal-expect: run
// tidal-note: x86-64. A class with a virtual function starts with a pointer to its table, so its first data member lies after that pointer.
struct Shape { virtual int sides() const { return 0; } int id = 5; };
int main() {
  Shape s;
  const char* base = reinterpret_cast<const char*>(&s);
  const char* id = reinterpret_cast<const char*>(&s.id);
  return id - base == sizeof(void*) && sizeof(Shape) == 2 * sizeof(void*) ? 0 : 1;
}
