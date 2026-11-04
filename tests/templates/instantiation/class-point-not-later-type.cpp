// tidal-rules: INST-33
// tidal-expect: reject
// tidal-error-clang: undeclared identifier 'probe'
// tidal-error-gcc: .probe. was not declared in this scope
// tidal-error-edg: identifier "probe" is undefined
// tidal-note: The member alias needs probe, and the only probe comes after the point of instantiation of Kind<lib::Item>.
namespace lib { struct Item {}; }
template <class T> struct Kind { using type = decltype(probe(T{})); };
Kind<lib::Item>::type x = 0;
namespace lib { int probe(Item); }
