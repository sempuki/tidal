// tidal-rules: TSP-18 LNK-7
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to
// tidal-error-gcc: undefined reference to
// tidal-note: Box<T*>::get is declared and not defined. The primary template's get does not supply it.
template <class T> struct Box { int get() { return 0; } };
template <class T> struct Box<T*> { int get(); };
int main() { Box<int*> b; return b.get(); }
