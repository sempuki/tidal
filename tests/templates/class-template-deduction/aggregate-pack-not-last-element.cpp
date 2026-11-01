// tidal-rules: CTAD-27 CTAD-9 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Tail" matches
// tidal-note: The pack of bases B is not at the end, so it takes no element, and the list has two elements for the one member x.
struct Empty {};
template <class... B> struct Tail : B... { int x; };
Tail t{Empty{}, 5};
