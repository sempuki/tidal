// tidal-rules: TARG-3
// tidal-expect: accept
// tidal-note: The > of a named cast inside an argument is nested. A >> token can close a template argument list and a named cast, or two template heads.
template <class T> struct Box { T value; };
template <int N> struct Bits { static constexpr int value = N; };
static_assert(Bits<static_cast<int>(2.5)>::value == 2);
static_assert(Bits<sizeof(static_cast<Box<int>>(Box<int>{}))>::value == sizeof(int));
const Box<int>* p = static_cast<const Box<int>*>(nullptr);
auto q = static_cast<Box<Box<int>>>(Box<Box<int>>{});
template <template <class> class C = Box> struct Use {};
template <template <class> class C> using Again = Use<C>;
