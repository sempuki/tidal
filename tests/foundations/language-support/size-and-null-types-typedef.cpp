// tidal-rules: LSUP-4
// tidal-expect: accept
// tidal-note: The names are declared with typedef. std::nullptr_t serves as a parameter type, and std::ptrdiff_t matches a difference of char pointers.
namespace std {
typedef decltype(sizeof 0) size_t;
typedef decltype(nullptr) nullptr_t;
typedef decltype(static_cast<char*>(nullptr) - static_cast<char*>(nullptr)) ptrdiff_t;
}
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
constexpr int which(std::nullptr_t) { return 1; }
constexpr int which(int*) { return 2; }
static_assert(which(nullptr) == 1);
char text[8];
static_assert(same<std::ptrdiff_t, decltype(&text[5] - &text[1])>);
static_assert(same<std::size_t, decltype(sizeof text)>);
