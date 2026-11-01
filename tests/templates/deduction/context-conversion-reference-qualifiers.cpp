// tidal-rules: DED-31
// tidal-expect: accept
// tidal-note: When both the result type and the needed type are references, the needed type keeps the const or volatile of its referred type, and T gets it. When only one of the two is a reference, the qualifiers go.
template <class T> constexpr int code = 0;
template <> constexpr int code<int> = 1;
template <> constexpr int code<const int> = 2;
template <> constexpr int code<volatile int> = 3;
int store = 0;
template <int Want> struct Ref { template <class T> operator T&() const { static_assert(code<T> == Want, "deduced"); return store; } };
template <int Want> struct Val { template <class T> operator T() const { static_assert(code<T> == Want, "deduced"); return T(); } };
template <int Want> struct Rref { template <class T> operator T&&() const { static_assert(code<T> == Want, "deduced"); return static_cast<T&&>(store); } };
const int& a = Ref<2>{};
volatile int& b = Ref<3>{};
const int c = Ref<1>{};
const int& d = Val<1>{};
const int&& e = Rref<2>{};
