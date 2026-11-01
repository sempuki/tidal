// tidal-rules: COND-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum E { red, green };
enum class Mode { on, off };
char letter = 'a';
const char fixed = 'b';
short count = 1;
bool flag = true;
static_assert(same<decltype(true ? letter : 'b'), char>);
static_assert(same<decltype(true ? fixed : 'c'), char>);
static_assert(same<decltype(true ? count : (short)2), short>);
static_assert(same<decltype(true ? true : false), bool>);
static_assert(same<decltype(true ? flag : 1), int>);
static_assert(same<decltype(true ? Mode::on : Mode::off), Mode>);
static_assert(same<decltype(true ? red : green), E>);
static_assert(same<decltype(true ? 1.0f : 2.0f), float>);
static_assert(same<decltype(true ? (char)1 : (unsigned char)2), int>);
