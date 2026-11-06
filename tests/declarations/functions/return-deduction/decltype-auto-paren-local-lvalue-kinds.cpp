// tidal-rules: RET-6 JMP-10
// tidal-expect: accept
// tidal-note: Decision 0059, item RD, with the round 9 writer probe autop3-ret2-implicit-move-limits.json. A parenthesized local object in a return statement is an xvalue, so decltype(auto) gives an rvalue reference. A volatile local and a local of lvalue reference type stay lvalues and give an lvalue reference. A local of rvalue reference type gives an rvalue reference. Only the types are read.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int global = 0;
decltype(auto) plain() { int x = 0; return (x); }
decltype(auto) shaky() { volatile int x = 0; return (x); }
decltype(auto) alias() { int& x = global; return (x); }
decltype(auto) moved() { int&& x = static_cast<int&&>(global); return (x); }
static_assert(same<decltype(plain()), int&&>);
static_assert(same<decltype(shaky()), volatile int&>);
static_assert(same<decltype(alias()), int&>);
static_assert(same<decltype(moved()), int&&>);
