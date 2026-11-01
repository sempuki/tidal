// tidal-rules: TARG-26
// tidal-expect: reject
// tidal-error-clang: is a private member of
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
// tidal-note: Peek receives Owner as an argument, but gets no access to its private members.
template <class T> struct Peek { static constexpr int size = sizeof(typename T::Hidden); };
class Owner { struct Hidden { int x; }; };
int size = Peek<Owner>::size;
