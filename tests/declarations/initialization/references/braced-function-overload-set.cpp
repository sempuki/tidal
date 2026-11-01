// tidal-rules: REF-6
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: invalid initialization of non-const reference
// tidal-note: Q-0165. The referenced type is a function type, so REF-6 binds the reference as = does, and the overload set chooses show(int). GCC treats the list as an rvalue.
void show(int) {}
void show(long) {}
void (&call)(int){show};
