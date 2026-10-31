// tidal-rules: INL-1
// tidal-expect: run
// tidal-sources: inline-internal-linkage.tu.cpp
static inline int local() { return 2; }
int other();
int main() { return local() + other() == 5 ? 0 : 1; }
