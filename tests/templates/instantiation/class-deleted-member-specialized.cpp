// tidal-rules: INST-5
// tidal-expect: accept
template <class T> struct Box { void reset() = delete; };
Box<int> b;
template <> void Box<int>::reset() {}
