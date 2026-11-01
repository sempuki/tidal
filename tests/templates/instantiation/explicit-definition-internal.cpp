// tidal-rules: INST-29
// tidal-expect: accept
template <class T> static int get(T) { return 1; }
template int get(int);
