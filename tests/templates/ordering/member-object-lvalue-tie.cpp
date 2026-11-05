// tidal-rules: ORD-6 ORD-15
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: ambiguous overload
// tidal-error-edg: more than one operator
// tidal-note: The member gets the object parameter Item&, the same type as the other template's first parameter. Both pairs are then the same, so neither template is more specialized.
struct Item { template <class T> int operator-(T); };
template <class T> int operator-(Item&, T);
Item item;
int use() { return item - 2; }
