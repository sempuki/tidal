// tidal-rules: VIRT-12
// tidal-expect: reject
// tidal-error-gcc: from outside current evaluation
// tidal-error-clang: delete of pointer to subobject
// tidal-error-edg: deallocation of storage that was not dynamically allocated
struct B { constexpr virtual ~B() {} };
struct D : B { int v = 0; };
constexpr bool f() { B* p = new D[2]; delete[] p; return true; }
static_assert(f());
