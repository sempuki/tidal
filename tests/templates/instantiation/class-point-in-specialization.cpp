// tidal-rules: INST-30
// tidal-expect: run
namespace lib { struct Item {}; }
template <class T> struct Info { static constexpr int value = probe(T{}); };
template <class T> int read() { return Info<T>::value; }
namespace lib { constexpr int probe(Item) { return 4; } }
int main() { return read<lib::Item>() == 4 ? 0 : 1; }
