// tidal-rules: DEP-5
// tidal-expect: run
int pick(double) { return 1; }
template <class T> int use(T) { return pick(1); }
int pick(int) { return 2; }
int main() { return use(0) == 1 ? 0 : 1; }
