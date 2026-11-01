// tidal-rules: INST-31 INST-33
// tidal-expect: reject
// tidal-error-clang: undeclared identifier 'probe'
// tidal-error-gcc: .probe. was not declared in this scope
// tidal-error-edg: identifier "probe" is undefined
namespace lib { struct Item {}; }
template <class T> constexpr int info(T item) { return probe(item); }
constexpr int x = info(lib::Item{});
namespace lib { constexpr int probe(Item) { return 4; } }
