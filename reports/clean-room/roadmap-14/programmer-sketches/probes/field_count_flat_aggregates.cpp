template <class T> T&& declval() noexcept;
struct any { template <class T> operator T() const; };
template <class T, class... A>
concept brace_init = requires { T{declval<A>()...}; };
template <class T, class... A>
constexpr int count_from() {
  if constexpr (brace_init<T, A..., any>)
    return count_from<T, A..., any>();
  else
    return sizeof...(A);
}
struct Two { int a, b; };
struct Four { int a; bool b; long long c; short d; };
static_assert(count_from<Two>() == 2);
static_assert(count_from<Four>() == 4);
int main() {}
