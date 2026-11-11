// tidal-rules: INST-33 INST-5
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: undeclared identifier 'probe'
// tidal-error-edg: identifier "probe" is undefined
// tidal-note: I6-15. The initializer in the class of a static const member that is not inline or constexpr is made with the class (Q-0433). The object early instantiates Info<lib::Item> before the declaration of probe, so the initializer does not find probe. GCC waits for a use of the member, and the read after probe finds it.
namespace lib { struct Item {}; }
template <class T> struct Info { static const int value = probe(T{}); };
Info<lib::Item> early;
namespace lib { constexpr int probe(Item) { return 4; } }
int late = Info<lib::Item>::value;
