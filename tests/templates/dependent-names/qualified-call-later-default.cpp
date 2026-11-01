// tidal-rules: DEP-31
// tidal-expect: run
// tidal-expect-clang: reject
// tidal-expect-edg: reject
// tidal-error-clang: too few arguments to function call
// tidal-error-edg: too few arguments in function call
// tidal-note: A declaration after the template adds the default argument that the qualified call needs. GCC uses the default, Clang and EDG reject the call. DEP-31 needs no diagnostic, so both results are allowed (Q-0249).
namespace lib { struct Item {}; int probe(Item, int); }
template <class T> int info(T item) { return lib::probe(item); }
namespace lib { int probe(Item, int = 4); }
int main() { return info(lib::Item{}) == 4 ? 0 : 1; }
namespace lib { int probe(Item, int n) { return n; } }
