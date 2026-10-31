// tidal-rules: CVF-3
// tidal-expect: accept
struct H { constexpr explicit operator bool() const { return true; } };
void f() noexcept(H{});
