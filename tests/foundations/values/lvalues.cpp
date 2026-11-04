// tidal-rules: VC-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x = 0;
int* p = &x;
int& ref();
static_assert(same<decltype((x)), int&>);
static_assert(same<decltype((*p)), int&>);
static_assert(same<decltype((ref())), int&>);
static_assert(same<decltype((++x)), int&>);
static_assert(same<decltype((x = 1)), int&>);
static_assert(same<decltype(("hi")), const char (&)[3]>);
int arr[2];
static_assert(same<decltype((arr[0])), int&>);
