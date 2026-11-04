// tidal-rules: COND-16 COND-3
// tidal-expect: accept
// tidal-note: The target for the class operand is the type of the array operand after the array becomes a pointer, so View converts to a const int* prvalue. The two operands are then pointers, and COND-3 gives a const int* prvalue.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
struct View { operator const int*() const; };
View v;
int row[3];
extern bool c;
static_assert(same<decltype(c ? v : row), const int*>);
