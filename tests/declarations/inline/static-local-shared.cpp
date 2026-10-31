// tidal-rules: INL-5
// tidal-expect: run
// tidal-sources: static-local-shared.tu.cpp
inline int tick() { static int n = 0; return ++n; }
int other();
int main() { tick(); other(); return tick() == 3 ? 0 : 1; }
