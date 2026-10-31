// tidal-rules: SA-3
// tidal-expect: accept
static_assert(5);
static_assert(nullptr == nullptr);
struct B { constexpr explicit operator bool() const { return true; } };
static_assert(B{});
