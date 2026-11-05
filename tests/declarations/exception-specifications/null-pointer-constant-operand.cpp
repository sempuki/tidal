// tidal-rules: EXS-7
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: to .bool. is not allowed in a converted constant
// tidal-note: Q-0393. The operand is a constant of the null pointer type. GCC and EDG convert it to false and accept. Clang rejects.
constexpr decltype(nullptr) none = nullptr;
void g() noexcept(none);
static_assert(!noexcept(g()));
