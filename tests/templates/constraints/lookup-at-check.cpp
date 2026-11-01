// tidal-rules: CNS-7
// tidal-expect: accept
// tidal-note: The functions use come after the concept. Argument-dependent lookup at the check finds lib::use for lib::Item. For int, ordinary lookup sees only the declarations before the concept, so it does not find ::use, and the requirement fails.
namespace lib { struct Item {}; }
template <class T> concept Usable = requires (T t) { use(t); };
namespace lib { void use(Item); }
void use(int);
static_assert(Usable<lib::Item>);
static_assert(!Usable<int>);
