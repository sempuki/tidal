// tidal-rules: INST-20 INST-16
// tidal-expect: run
// tidal-sources: explicit-definition-before-template-definition.tu.cpp
template <class T> int size(T);
extern template int size(int);
int main() { return size(1) == sizeof(int) ? 0 : 1; }
