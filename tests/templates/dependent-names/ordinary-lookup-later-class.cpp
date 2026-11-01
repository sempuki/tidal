// tidal-rules: DEP-10
// tidal-expect: reject
// tidal-error-clang: neither visible in the template definition nor found by argument-dependent lookup
// tidal-error-gcc: no declarations were found by argument-dependent lookup
// tidal-error-edg: identifier "probe" is undefined
namespace lib {
struct Item {};
}
template <class T> int use(T item) { return probe(item); }
int probe(lib::Item) { return 7; }
int x = use(lib::Item{});
