// tidal-rules: CNS-35
// tidal-expect: reject
// tidal-error-clang: depends on itself
// tidal-error-gcc: depends on itself
// tidal-error-edg: depends on itself
// tidal-note: Usable<Item> needs a call of use, and argument-dependent lookup finds the template use, whose constraint is Usable<Item> again.
template <class T> concept Usable = requires (T t) { use(t); };
template <Usable T> void use(T);
struct Item {};
bool usable = Usable<Item>;
