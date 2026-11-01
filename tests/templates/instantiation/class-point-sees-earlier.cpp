// tidal-rules: INST-30
// tidal-expect: run
namespace lib { struct Item {}; }
template <class T> struct Info { static constexpr int value = probe(T{}); };
namespace lib { constexpr int probe(Item) { return 4; } }
int x = Info<lib::Item>::value;
int main() { return x == 4 ? 0 : 1; }
