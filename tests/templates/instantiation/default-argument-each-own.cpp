// tidal-rules: INST-9
// tidal-expect: run
template <class T> int take(T x, int y = T::nothing, int z = 3) { return y + z; }
int main() { return take(1, 2) == 5 ? 0 : 1; }
