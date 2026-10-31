// tidal-rules: PRIM-10
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(true), bool>);
static_assert(same<decltype(nullptr), decltype(nullptr)>);
static_assert(sizeof(nullptr) == sizeof(void*));
int* p = nullptr;
void (*fp)() = nullptr;
bool b(nullptr);
