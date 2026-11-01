// tidal-rules: INST-30 INST-33
// tidal-expect: reject
// tidal-error-clang: undeclared identifier 'probe'
// tidal-error-gcc: .probe. was not declared in this scope
// tidal-error-edg: identifier "probe" is undefined
namespace lib { struct Item {}; }
template <class T> struct Info { static constexpr int value = probe(T{}); };
int x = Info<lib::Item>::value;
namespace lib { constexpr int probe(Item) { return 4; } }
