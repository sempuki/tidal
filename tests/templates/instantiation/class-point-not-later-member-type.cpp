// tidal-rules: INST-33 INST-30
// tidal-expect: reject
// tidal-error-clang: undeclared identifier 'probe'
// tidal-error-gcc: .probe. was not declared in this scope
// tidal-error-edg: identifier "probe" is undefined
// tidal-note: The object early needs Info<lib::Item>, whose point of instantiation comes before the declaration of probe. The type of the member value is made with the class and needs probe, so the class is not valid. The later declaration does not help.
namespace lib { struct Item {}; }
template <class T> struct Info { decltype(probe(T{})) value; };
Info<lib::Item> early;
namespace lib { constexpr int probe(Item) { return 4; } }
