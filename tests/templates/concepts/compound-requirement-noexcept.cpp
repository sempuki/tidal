// tidal-rules: CPT-33
// tidal-expect: accept
void quiet() noexcept;
void loud();
template <class F> concept NoThrow = requires (F f) { { f() } noexcept; };
static_assert(NoThrow<decltype(&quiet)> && !NoThrow<decltype(&loud)>);
