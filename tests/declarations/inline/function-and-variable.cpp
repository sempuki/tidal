// tidal-rules: INL-1
// tidal-expect: run
inline int counter = 1;
inline int next() { return ++counter; }
int main() { return next() == 2 ? 0 : 1; }
