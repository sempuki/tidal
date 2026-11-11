// The uz suffix gives std::size_t (C++23).
template <class A, class B> inline constexpr bool same = false;
template <class A> inline constexpr bool same<A, A> = true;
static_assert(same<decltype(5uz), decltype(sizeof 0)>);
int main() { auto n = 3uz; return static_cast<int>(n) - 3; }
