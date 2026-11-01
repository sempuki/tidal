// tidal-rules: INST-4 INST-2
// tidal-expect: accept
template <class T> struct Later;
Later<int>* p = nullptr;
template <class T> struct Later { int v = 1; };
Later<int> y;
