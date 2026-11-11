// tidal-rules: INST-33 INST-5 INST-8
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: identifier "probe" is undefined
// tidal-note: I6-15. The object early instantiates Info<lib::Item> before the declaration of probe. The initializer of the static constexpr member value waits for a use of the member, and the read after probe makes it, so the read finds probe. EDG makes the initializer with the class and rejects the object (Q-0231).
namespace lib { struct Item {}; }
template <class T> struct Info { static constexpr int value = probe(T{}); };
Info<lib::Item> early;
namespace lib { constexpr int probe(Item) { return 4; } }
static_assert(Info<lib::Item>::value == 4);
