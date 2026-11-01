// tidal-rules: LIST-6 REF-14
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: invalid conversion from .<brace-enclosed initializer list>.
// tidal-note: Q-0166. Each literal needs a temporary array, which the literal initializes under LIST-6. GCC rejects a braced string literal that does not bind the reference directly.
constexpr const unsigned char (&high)[3]{"ab"};
constexpr const signed char (&low)[3]{"ab"};
constexpr const char (&room)[4]{"ab"};
constexpr const char (&utf)[3]{u8"ab"};
static_assert(high[0] == 'a' && high[2] == 0);
static_assert(low[1] == 'b');
static_assert(room[2] == 0 && room[3] == 0);
static_assert(utf[0] == 'a');
