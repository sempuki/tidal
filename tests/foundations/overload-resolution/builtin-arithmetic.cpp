// tidal-rules: OVL-9
// tidal-expect: accept
// tidal-note: Each class converts to one arithmetic type. The built-in candidate whose parameters match the converted types exactly wins, and its result type is the type of the expression.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct ToInt { operator int() const; };
struct ToShort { operator short() const; };
struct ToDouble { operator double() const; };
static_assert(same<decltype(ToInt{} * ToDouble{}), double>);
static_assert(same<decltype(ToShort{} + ToShort{}), int>);
static_assert(same<decltype(ToInt{} < ToDouble{}), bool>);
static_assert(same<decltype(-ToShort{}), int>);
static_assert(same<decltype(+ToDouble{}), double>);
static_assert(same<decltype(~ToShort{}), int>);
static_assert(same<decltype(ToInt{} % 3L), long>);
static_assert(same<decltype(ToShort{} << 1L), int>);
static_assert(same<decltype(ToInt{} & ToShort{}), int>);
static_assert(same<decltype(!ToDouble{}), bool>);
static_assert(same<decltype(ToInt{} && ToDouble{}), bool>);
