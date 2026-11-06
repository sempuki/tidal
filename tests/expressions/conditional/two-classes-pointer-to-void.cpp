// tidal-rules: COND-11
// tidal-expect: accept
// tidal-note: Decision 0059, item F2 (I4-11). The pointer candidate that wins is the one for the composite pointer type of the types that the operands reach: void* for int* and void*, and const void* for const int* and void*.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
template <class T> T&& move_of(T& t) { return static_cast<T&&>(t); }
struct ToInt { operator int*(); };
struct ToConstInt { operator const int*(); };
struct ToVoid { operator void*(); };
ToInt pi; ToConstInt pci; ToVoid pv;
extern bool c;
static_assert(same<decltype(c ? pi : pv), void*>);
static_assert(same<decltype(c ? pci : pv), const void*>);
static_assert(same<decltype(c ? pi : pci), const int*>);
