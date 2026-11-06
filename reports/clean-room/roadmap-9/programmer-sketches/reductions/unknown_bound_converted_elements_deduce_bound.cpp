// An array of unknown bound whose elements convert through a converting
// constructor, passed to a function template that deduces the bound (DCL-5,
// DED-1). P1-1 found the same array stop as "unsupported".
template <class T, decltype(sizeof 0) N> constexpr decltype(sizeof 0) len(T (&)[N]) { return N; }
struct S { S(const char*) {} };
int main() { S a[] = {"x", "y", "z"}; return len(a) == 3 ? 0 : 1; }
