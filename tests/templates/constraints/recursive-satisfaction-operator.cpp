// tidal-rules: CNS-35
// tidal-expect: reject
// tidal-error-clang: depends on itself
// tidal-error-gcc: depends on itself
// tidal-error-edg: depends on itself
// tidal-note: Addable<Item> needs a + a. Lookup finds the operator template, whose constraint is Addable<Item> again.
template <class T> concept Addable = requires (T a) { a + a; };
template <Addable T> T operator+(T, T);
struct Item {};
bool addable = Addable<Item>;
