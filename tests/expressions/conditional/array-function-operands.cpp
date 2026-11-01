// tidal-rules: COND-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int three[3];
int four[4];
void act();
void other();
static_assert(same<decltype(true ? three : four), int*>);
static_assert(same<decltype(true ? "ab" : "xyz"), const char*>);
static_assert(same<decltype(true ? "ab" : "cd"), const char (&)[3]>);
static_assert(same<decltype(true ? act : other), void (&)()>);
static_assert(same<decltype(true ? act : &other), void (*)()>);
