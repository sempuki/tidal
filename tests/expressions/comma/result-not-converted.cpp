// tidal-rules: COMMA-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int cells[10];
void act();
static_assert(sizeof((0, cells)) == sizeof(cells));
static_assert(same<decltype((0, cells)), int(&)[10]>);
static_assert(same<decltype((0, act)), void(&)()>);
