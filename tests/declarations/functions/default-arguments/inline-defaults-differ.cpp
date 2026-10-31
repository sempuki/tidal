// tidal-rules: DEFARG-7
// tidal-expect: run
// tidal-sources: inline-defaults-differ.tu.cpp
// tidal-note: Gap review 2026-10-31. The two units give the parameter different defaults. No compiler or linker reports it; the test accepts either value.
inline int pick(int v = 1) { return v; }
int other();
int main() { int a = pick(); int b = other(); return (a == 1 || a == 2) && (b == 1 || b == 2) ? 0 : 1; }
