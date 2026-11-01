// tidal-rules: INST-32 INST-31
// tidal-expect: run
// tidal-expect-clang: reject
// tidal-expect-edg: reject
// tidal-error-clang: use of undeclared identifier .probe.
// tidal-error-edg: identifier "probe" is undefined
// tidal-note: info<lib::Item> is constexpr and used outside constant evaluation. Right after use, probe is not declared. At the end of the unit, probe is declared. The points differ, so INST-32 allows any result. GCC takes the end of the unit. Clang and EDG instantiate at the first point and reject (Q-0310).
namespace lib { struct Item {}; }
template <class T> constexpr int info(T item) { return probe(item); }
int use() { return info(lib::Item{}); }
namespace lib { int probe(Item) { return 4; } }
int main() { return use() == 4 ? 0 : 1; }
