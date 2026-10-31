// tidal-rules: NAM-13
// tidal-expect: accept
namespace lib { inline namespace v2 { inline namespace detail { int value = 2; } } }
int read() { return lib::value + lib::v2::value + lib::v2::detail::value; }
