// tidal-rules: TSP-14
// tidal-expect: accept
// tidal-note: The partial specialization comes after the using-declaration and is still selected through the using name.
namespace lib { template <class T> struct Kind { static constexpr int value = 0; }; }
using lib::Kind;
namespace lib { template <class T> struct Kind<T*> { static constexpr int value = 1; }; }
static_assert(Kind<int*>::value == 1);
