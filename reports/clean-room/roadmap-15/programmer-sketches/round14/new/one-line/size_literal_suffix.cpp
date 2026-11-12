template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using size_t = decltype(sizeof 0);
static_assert(same<decltype(5uz), size_t>);
int main() { size_t n = 0uz; for (auto i = 0uz; i < 4uz; ++i) n += i; return static_cast<int>(n) - 6; }
