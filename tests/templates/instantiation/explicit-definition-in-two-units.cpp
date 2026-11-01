// tidal-rules: INST-35
// tidal-expect: run
// tidal-sources: explicit-definition-in-two-units.tu.cpp
// tidal-note: Both units have the explicit instantiation definition of size<int>. INST-35 needs no diagnostic, and every compiler builds the program.
template <class T> int size(T value) { return sizeof(value); }
template int size(int);
int there();
int main() { return size(1) == sizeof(int) && there() == sizeof(int) ? 0 : 1; }
