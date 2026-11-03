// tidal-rules: EXS-2
// tidal-expect: accept
// tidal-note: A noexcept argument matches the noexcept overload exactly. The other overload needs the step that drops noexcept, so it is worse (ICS-26).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Plain {};
struct NoThrow {};
Plain take(void (*)());
NoThrow take(void (*)() noexcept);
void g() noexcept;
void h();
static_assert(same<decltype(take(g)), NoThrow>);
static_assert(same<decltype(take(h)), Plain>);
