// tidal-rules: TARG-14
// tidal-expect: accept
// tidal-note: The addresses of a global, a static local, a function, an object with internal linkage, and the null pointer.
template <const int* P> struct Ptr { static constexpr const int* value = P; };
template <int (*F)()> struct Fn {};
int global = 1;
static int hidden = 2;
int answer() { return 1; }
constexpr const int* address() { static const int kept = 3; return &kept; }
Ptr<&global> a;
Ptr<&hidden> b;
Ptr<nullptr> c;
Ptr<address()> d;
Fn<answer> e;
Fn<&answer> f;
static_assert(Ptr<nullptr>::value == nullptr);
static_assert(Ptr<&global>::value == &global);
