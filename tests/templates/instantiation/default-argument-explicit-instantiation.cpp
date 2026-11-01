// tidal-rules: INST-9
// tidal-expect: accept
template <class T> int take(T x, int y = T::nothing) { return y; }
template int take<int>(int, int);
template <class T> struct Box { int take(int y = T::nothing) { return y; } };
template struct Box<int>;
