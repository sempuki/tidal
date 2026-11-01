// tidal-rules: CE-8 VIRT-10
// tidal-expect: reject
// tidal-error-clang: reference dynamic_cast failed
// tidal-error-gcc: reference .dynamic_cast. failed
// tidal-error-edg: dynamic_cast to subobject of type .* is invalid
struct A { virtual constexpr ~A() {} };
struct D : A {};
constexpr bool fails() { A plain; (void)dynamic_cast<D&>(plain); return true; }
static_assert(fails());
