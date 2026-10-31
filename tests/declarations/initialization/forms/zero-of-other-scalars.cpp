// tidal-rules: INIT-2
// tidal-expect: accept
enum E { a = 1, b = 2 };
enum class F { x = 3 };
struct S { int m; };
constexpr E e{};
constexpr F f{};
constexpr int S::* pm{};
constexpr decltype(nullptr) np{};
static_assert(e == 0 && int(f) == 0 && pm == nullptr && np == nullptr);
