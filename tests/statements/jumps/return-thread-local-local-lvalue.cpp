// tidal-rules: JMP-10 RET-6
// tidal-expect: accept
// tidal-note: Gap review RV6-A-9. A thread_local variable at block scope has thread storage, so JMP-10 does not count it, and its name in a return statement stays an lvalue. The decltype(auto) function returns int&, and every compiler rejects a return of a move-only thread_local local by name (writer probe rv6-points.json).
template <class A, class B> struct same { static constexpr bool value = false; };
template <class A> struct same<A, A> { static constexpr bool value = true; };
decltype(auto) counter() { thread_local int count = 0; return (count); }
static_assert(same<decltype(counter()), int&>::value);
int main() { counter() = 3; return counter() == 3 ? 0 : 1; }
