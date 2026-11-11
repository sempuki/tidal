// A constant template parameter declared auto, with two argument types.
template <auto V> struct Constant { static constexpr auto value = V; };
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(Constant<'a'>::value), const char>);
static_assert(same<decltype(Constant<3L>::value), const long>);
int main() { return Constant<7>::value - 7; }
