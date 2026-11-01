// tidal-rules: TARG-26
// tidal-expect: reject
// tidal-error-clang: is a private member of
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
template <class T> struct Box {};
class Owner { struct Hidden {}; };
Box<Owner::Hidden> b;
