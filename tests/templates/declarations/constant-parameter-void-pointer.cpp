// tidal-rules: TPL-6
// tidal-expect: accept
// tidal-note: Constant parameters of type void* and const volatile void*. The argument already has the parameter's type.
template <void* P> struct Raw { static constexpr void* address = P; };
template <const volatile void* P> struct Any {};
int global;
constexpr void* erased = &global;
Raw<erased> a;
Raw<nullptr> b;
Any<static_cast<const volatile void*>(&global)> c;
static_assert(Raw<erased>::address == &global);
