// tidal-rules: CX-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
// tidal-sources: consteval-inline.tu.cpp
consteval int twice(int v) { return v * 2; }
int other();
int main() { return twice(2) == 4 && other() == 6 ? 0 : 1; }
