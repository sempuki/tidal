// tidal-rules: CNS-10
// tidal-expect: accept
// tidal-note: Usable<Item> is checked before and after a function use for Item is declared. The result changes, so the program is not valid. CNS-10 needs no diagnostic, and every compiler accepts.
template <class T> concept Usable = requires (T t) { use(t); };
struct Item {};
constexpr bool before = Usable<Item>;
void use(Item);
constexpr bool after = Usable<Item>;
static_assert(!before);
