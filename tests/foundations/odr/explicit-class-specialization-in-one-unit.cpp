// tidal-rules: ODR-14
// tidal-expect: run
// tidal-sources: explicit-class-specialization-in-one-unit.tu.cpp
// tidal-note: Only this unit declares the explicit specialization Slot<char>. ODR-14 needs no diagnostic, and the program accepts the result of either definition.
template <class T> struct Slot { static int code() { return 0; } };
template <> struct Slot<char> { static int code() { return 1; } };
int code_elsewhere();
int main() {
  int here = Slot<char>::code(), there = code_elsewhere();
  return (here == 0 || here == 1) && (there == 0 || there == 1) ? 0 : 1;
}
