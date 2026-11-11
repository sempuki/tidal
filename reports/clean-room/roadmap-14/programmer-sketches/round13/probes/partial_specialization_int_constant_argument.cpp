template <int V, class T = int> struct Q {};
template <class X> struct is_one { static constexpr bool value = false; };
template <class T> struct is_one<Q<1, T>> { static constexpr bool value = true; };
int main() { return (is_one<Q<1>>::value ? 0 : 1) + (is_one<Q<2>>::value ? 2 : 0); }
