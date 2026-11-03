// tidal-rules: INST-32 INST-31
// tidal-expect: reject
// tidal-error-clang: undeclared identifier 'probe'
// tidal-error-gcc: .probe. was not declared in this scope
// tidal-error-edg: identifier "probe" is undefined
// tidal-note: Q-0257. The constant x evaluates info<lib::Item> at the first point, where probe is not declared. At the end of the unit, probe is declared. The points differ, so INST-32 allows any outcome. Every compiler rejects.
namespace lib { struct Item {}; }
template <class T> constexpr int info(T item) { return probe(item); }
constexpr int x = info(lib::Item{});
namespace lib { constexpr int probe(Item) { return 4; } }
