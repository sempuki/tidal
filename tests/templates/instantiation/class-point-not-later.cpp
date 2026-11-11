// tidal-rules: INST-32 INST-31
// tidal-expect: reject
// tidal-error-clang: undeclared identifier 'probe'
// tidal-error-gcc: .probe. was not declared in this scope
// tidal-error-edg: identifier "probe" is undefined
// tidal-note: Q-0257. The read of Info<lib::Item>::value makes the initializer of the static constexpr member, and its constant evaluation runs at the member's first point of instantiation, right after the declaration of x, where probe is not declared. At the end of the unit, probe is declared. The points differ, so INST-32 allows any outcome. Every compiler rejects.
namespace lib { struct Item {}; }
template <class T> struct Info { static constexpr int value = probe(T{}); };
int x = Info<lib::Item>::value;
namespace lib { constexpr int probe(Item) { return 4; } }
