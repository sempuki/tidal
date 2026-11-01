// tidal-rules: INST-35
// tidal-expect: run
// tidal-sources: explicit-specialization-in-other-unit.tu.cpp
// tidal-note: This unit has an explicit instantiation definition of size<int>, and the other unit has an explicit specialization. INST-35 needs no diagnostic, and the program accepts either definition.
template <class T> int size(T value) { return sizeof(value); }
template int size(int);
int there();
int main() {
  int a = size(1), b = there();
  return (a == sizeof(int) || a == 99) && (b == sizeof(int) || b == 99) ? 0 : 1;
}
