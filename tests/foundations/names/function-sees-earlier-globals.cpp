// tidal-rules: NAM-1
// tidal-expect: run
int g = 5;
int get() { return g; }
int main() { return get() == 5 ? 0 : 1; }
