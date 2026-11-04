// tidal-rules: CE-13
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: comparison of addresses of (potentially overlapping )?literals
// tidal-note: Q-0033. The literals are held in two constexpr pointers, and the comparison is in a static_assert. Clang rejects; GCC and EDG fold it.
constexpr const char* p = "xy";
constexpr const char* q = "xy";
static_assert(p == q || p != q);
