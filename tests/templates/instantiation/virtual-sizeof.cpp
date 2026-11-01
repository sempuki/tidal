// tidal-rules: INST-11
// tidal-expect: accept
// tidal-note: The class is instantiated for sizeof. No compiler instantiates the virtual function, which INST-11 allows.
template <class T> struct Box { virtual int get() { return T::nothing; } };
int size = sizeof(Box<int>);
