// tidal-rules: COND-8
// tidal-expect: reject
// tidal-error-clang: call to deleted constructor
// tidal-error-gcc: use of deleted function
// tidal-error-edg: it is a deleted function
// tidal-note: Decision 0059, item F2 (I4-9), I4 program p9-lvalue-xvalue-deleted-move.cpp. An lvalue and an xvalue of one class give a prvalue. The xvalue operand is moved into the result, and the move constructor is deleted, so the expression is not valid even in an unevaluated operand.
template <class T> T&& move_of(T& t) { return static_cast<T&&>(t); }
struct Pinned {
  Pinned() = default;
  Pinned(const Pinned&) = default;
  Pinned(Pinned&&) = delete;
};
Pinned a, b;
extern bool c;
using R = decltype(c ? a : move_of(b));
