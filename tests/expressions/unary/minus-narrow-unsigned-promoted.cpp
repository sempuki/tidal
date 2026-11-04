// tidal-rules: UN-3 UN-1
// tidal-expect: accept
// tidal-note: An unsigned char or unsigned short operand promotes to int before the negation, so the result is the int -1, and not 2^N - 1 for the width of the operand's own type.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
constexpr unsigned char uc = 1;
constexpr unsigned short us = 1;
static_assert(same<decltype(-uc), int>);
static_assert(-uc == -1);
static_assert(same<decltype(-us), int>);
static_assert(-us == -1);
static_assert(-static_cast<unsigned char>(200) == -200);
