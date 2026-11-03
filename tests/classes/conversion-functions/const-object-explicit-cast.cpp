// tidal-rules: CVF-9
// tidal-expect: reject
// tidal-error-clang: cannot cast from type .const S. to pointer type
// tidal-error-gcc: invalid .static_cast. from type .const S.
// tidal-error-edg: no suitable conversion function from .const S.
// tidal-note: A static_cast of a const object to a pointer type. The only conversion function is not const.
struct S { operator int*() { return nullptr; } };
const S s{};
int* p = static_cast<int*>(s);
