// tidal-rules: ALN-6
// tidal-expect: run
// tidal-sources: different-units-alignas.tu.cpp
alignas(16) extern int shared;
int get();
int main() { return get() == 0 ? 0 : 1; }
