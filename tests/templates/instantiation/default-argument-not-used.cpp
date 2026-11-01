// tidal-rules: INST-9
// tidal-expect: accept
template <class T> int pick(T value, int extra = T::nothing) { return extra; }
int x = pick(1, 2);
