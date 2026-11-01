// tidal-rules: TPL-8
// tidal-expect: accept
// tidal-note: An array type and a function type become pointer types.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
int table[4];
int answer() { return 42; }
template <int A[4]> struct Array { using type = decltype(A); };
template <int F()> struct Function { using type = decltype(F); };
static_assert(same<Array<table>::type, int*>);
static_assert(same<Function<answer>::type, int (*)()>);
