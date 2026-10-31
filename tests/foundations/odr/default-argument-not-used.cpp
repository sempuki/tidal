// tidal-rules: ODR-6
// tidal-expect: run
int seed();
int pick(int v = seed()) { return v; }
int main() { return pick(0); }
