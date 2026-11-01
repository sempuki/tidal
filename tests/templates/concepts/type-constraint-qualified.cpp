// tidal-rules: CPT-17
// tidal-expect: accept
namespace lib { template <class T> concept Big = sizeof(T) > 1; }
template <lib::Big T> constexpr int one = 1;
constexpr int take(lib::Big auto x) { return one<decltype(x)>; }
template <class T> concept Takes = requires (T t) { take(t); };
static_assert(take(1) == 1 && !Takes<char>);
