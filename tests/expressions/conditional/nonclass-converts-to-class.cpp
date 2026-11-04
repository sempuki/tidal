// tidal-rules: COND-16 COND-3
// tidal-expect: accept
// tidal-note: The int operand converts to Meter through the converting constructor. Meter has no conversion to int, so the result is a Meter prvalue.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
struct Meter { Meter(int); };
Meter m(0);
extern bool c;
static_assert(same<decltype(c ? m : 1), Meter>);
static_assert(same<decltype(c ? 1 : m), Meter>);
