template <class T> T&& declval() noexcept;
struct any { template <class T> operator T() const; };
struct P { int x; long y; };
template <class T> constexpr bool two = requires { T{declval<any>(), declval<any>()}; };
static_assert(two<P>);
int main() {}
