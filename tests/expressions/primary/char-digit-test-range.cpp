// tidal-rules: PRIM-31
// tidal-expect: accept
// tidal-note: Because the digits are consecutive and increasing, one range check finds a digit, and digit + n gives the digit n places on.
constexpr bool is_digit(char c) { return c >= '0' && c <= '9'; }
constexpr char digit_char(int n) { return static_cast<char>('0' + n); }
static_assert(is_digit('0') && is_digit('5') && is_digit('9'));
static_assert(!is_digit('a') && !is_digit(' ') && !is_digit('/') && !is_digit(':'));
static_assert(digit_char(0) == '0' && digit_char(7) == '7' && digit_char(9) == '9');
static_assert('3' + 4 == '7' && '9' - 9 == '0');
static_assert('0' < '1' && '8' < '9');
