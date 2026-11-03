// tidal-rules: DEFARG-7
// tidal-expect: run
// tidal-sources: inline-default-only-one-unit.tu.cpp
// tidal-note: Only this unit gives the second parameter a default. The other unit declares the inline function without it. No compiler or linker reports the difference, and the test accepts the program's result.
inline int scale(int v, int f = 2) { return v * f; }
int other();
int main() { return scale(3) == 6 && other() == 12 ? 0 : 1; }
