// tidal-rules: DEP-10
// tidal-expect: reject
// tidal-error-clang: neither visible in the template definition nor found by argument-dependent lookup
// tidal-error-gcc: no match for .operator\+.
// tidal-error-edg: no operator "\+" matches these operands
template <class T> int add(T a, int b) { return a + b; }
namespace shop { struct Item {}; }
int operator+(shop::Item, int) { return 7; }
int x = add(shop::Item{}, 1);
