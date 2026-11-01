// tidal-rules: ICS-38
// tidal-expect: accept
// tidal-note: One string literal in braces has the identity to an array of its character type. A wide literal has no sequence to a char array, so the user-defined sequence to Wide wins.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Wide { Wide(const wchar_t*); };
struct Text { Text(const char*); };
One narrow(const char (&)[3]);
Two narrow(Wide);
static_assert(same<decltype(narrow({L"ab"})), Two>);
One plain(const char (&)[3]);
Two plain(Text);
static_assert(same<decltype(plain({"ab"})), One>);
One kind(const char (&)[3]);
Two kind(const wchar_t (&)[3]);
static_assert(same<decltype(kind({"ab"})), One>);
