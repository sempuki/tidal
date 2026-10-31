// tidal-rules: TYP-6
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<unsigned, unsigned int>);
static_assert(same<signed, int> && same<signed int, int>);
static_assert(same<short, short int> && same<signed short int, short>);
static_assert(same<long unsigned, unsigned long int>);
static_assert(same<int long long unsigned, unsigned long long>);
static_assert(same<long int signed, long>);
static_assert(same<signed char, char signed>);
