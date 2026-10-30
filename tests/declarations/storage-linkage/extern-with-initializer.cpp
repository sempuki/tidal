// tidal-rules: STO-7
// tidal-expect: run
// tidal-sources: extern-with-initializer.tu.cpp
extern int defined_with_extern = 42;
int read_from_other_unit();
int main() { return read_from_other_unit() == 42 ? 0 : 1; }
