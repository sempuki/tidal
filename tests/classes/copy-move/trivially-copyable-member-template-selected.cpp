// tidal-rules: CPY-27 UNI-11
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: implicitly deleted
// tidal-error-gcc: union member .* with non-trivial
// tidal-note: Q-0339. Pair needs a non-const source, because Plain copies only from one, so its implicit copy constructor copies part from a non-const Part. The constructor template of Part is then selected, and it is not trivial, so the copy constructor of Pair is not trivial and UNI-3 deletes the union's copy constructor. Clang and GCC delete it. EDG looks only at the copy constructors of Part, which are trivial, and copies the union.
template <class T> union Wrap { T x; };
struct Part { Part() = default; template <class T> Part(T&) {} };
struct Plain { Plain() = default; Plain(Plain&) = default; };
struct Pair { Pair() = default; Part part; Plain plain; };
void copy(Wrap<Pair>& from) { new Wrap<Pair>(from); }
