// tidal-rules: STG-1
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-flags: -m32
// tidal-error-gcc: overflow in constant expression
// tidal-error-edg: class is too large
// tidal-note: Q-0350. The class has 3 * 0x60000000 bytes, more than std::size_t holds with -m32. Clang 22 and 18 accept the class and give sizeof the size reduced modulo 2^32.
static_assert(sizeof(void*) == 4);
struct Many { char a[0x60000000], b[0x60000000], c[0x60000000]; };
constexpr auto size = sizeof(Many);
