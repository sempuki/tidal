// tidal-rules: INL-7
// tidal-expect: run
// tidal-expect-gcc-16-o2: link-fail
// tidal-expect-clang-22-o2: link-fail
// tidal-sources: inline-in-one-unit-only.tu.cpp
// tidal-note: At -O2, GCC and Clang inline f in main and emit no out-of-line definition, so the other unit's call does not link (INL-7 needs no diagnostic).
inline int f() { return 1; }
int other();
int main() { return f() + other() == 2 ? 0 : 1; }
