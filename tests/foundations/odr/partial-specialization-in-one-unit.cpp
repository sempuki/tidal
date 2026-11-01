// tidal-rules: ODR-14
// tidal-expect: run
// tidal-sources: partial-specialization-in-one-unit.tu.cpp
// tidal-note: Only this unit declares Shape<T*>. ODR-14 needs no diagnostic, and the program accepts the result of either definition.
template <class T> struct Shape { static int code() { return 0; } };
template <class T> struct Shape<T*> { static int code() { return 1; } };
int code_elsewhere();
int main() {
  int here = Shape<int*>::code(), there = code_elsewhere();
  return (here == 0 || here == 1) && (there == 0 || there == 1) ? 0 : 1;
}
