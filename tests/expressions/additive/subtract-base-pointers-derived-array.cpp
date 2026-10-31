// tidal-rules: ADD-14
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: subtracted pointers are not elements of the same array
// tidal-error-edg: invalid arithmetic on non-array pointer
// tidal-note: Q-0049. GCC accepts the subtraction during constant evaluation. Clang and EDG reject it.
// tidal-note: Gap review 2026-10-31 (third).
struct B { int a = 0; };
struct D : B { int b = 0; };
constexpr D arr[3]{};
constexpr const B* p = &arr[0];
constexpr const B* q = &arr[2];
constexpr auto d = q - p;
