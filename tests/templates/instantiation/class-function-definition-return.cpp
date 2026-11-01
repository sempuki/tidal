// tidal-rules: INST-1 INST-6
// tidal-expect: reject
// tidal-error: static assertion failed
template <class T> struct Box { static_assert(sizeof(T) == 0); };
Box<int> make() { while (true) {} }
