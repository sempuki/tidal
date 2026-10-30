// tidal-rules: ENUM-6
// tidal-expect: accept
enum E : unsigned char { A = 1 };
constexpr E e = static_cast<E>(200);
enum class F { B };
constexpr F f = static_cast<F>(1000);
static_assert(static_cast<int>(e) == 200 && static_cast<int>(f) == 1000);
