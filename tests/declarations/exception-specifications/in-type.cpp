// tidal-rules: EXS-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using F = void() noexcept;
using G = void();
static_assert(!same<F, G>);
void (*p)() noexcept = nullptr;
struct S { void m() noexcept; };
void (S::* pm)() noexcept = &S::m;
