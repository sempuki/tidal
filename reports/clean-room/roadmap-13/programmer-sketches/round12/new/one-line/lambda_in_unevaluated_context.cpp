// A lambda in decltype gives a distinct type each time (C++20).
template <class T = decltype([] {})> struct Tag {};
template <class A, class B> inline constexpr bool same = false;
template <class A> inline constexpr bool same<A, A> = true;
static_assert(!same<Tag<>, Tag<>>);
int main() { using F = decltype([](int x) { return x + 1; }); return F{}(-1); }
