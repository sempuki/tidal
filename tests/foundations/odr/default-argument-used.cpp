// tidal-rules: ODR-6 ODR-8
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .seed\(\)
// tidal-error-gcc: undefined reference to .seed\(\)
int seed();
int pick(int v = seed()) { return v; }
int main() { return pick(); }
