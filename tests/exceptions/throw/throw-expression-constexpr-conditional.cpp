// tidal-rules: THR-5
// tidal-expect: accept
// tidal-note: A throw expression as the second operand of the conditional operator in a constexpr function, and as the operand of return in a void function.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(throw "stop"), void>);
constexpr int pick(bool fail) { return fail ? throw 1 : 5; }
static_assert(pick(false) == 5);
void stop() { return throw 2; }
