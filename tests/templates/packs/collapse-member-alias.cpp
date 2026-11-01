// tidal-rules: PACK-24
// tidal-expect: accept
// tidal-note: The member alias adds & to T, and a template alias adds const& to T. A reference in T collapses the added one, and const on the reference type is dropped.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Ref { using type = T&; };
static_assert(same<Ref<int&&>::type, int&> && same<Ref<int&>::type, int&> && same<Ref<int>::type, int&>);
template <class T> using ConstRef = const T&;
static_assert(same<ConstRef<int&>, int&> && same<ConstRef<int&&>, int&>);
int&& make();
static_assert(same<decltype(make())&&, int&&> && same<decltype(make())&, int&>);
