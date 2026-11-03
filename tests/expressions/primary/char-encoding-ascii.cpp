// tidal-rules: PRIM-32
// tidal-expect: accept
// tidal-note: ASCII characters only, so each matrix target and configuration agrees. Control characters, punctuation, digits, and letters have their ASCII values in the ordinary and the wide encoding.
static_assert('\a' == 7 && '\r' == 13 && ' ' == 32 && '!' == 33 && '/' == 47);
static_assert('0' == 48 && ':' == 58 && '@' == 64 && 'A' == 65 && 'Z' == 90);
static_assert('[' == 91 && '`' == 96 && 'a' == 97 && 'z' == 122 && '{' == 123 && '~' == 126);
static_assert(L'\a' == 7 && L'\r' == 13 && L' ' == 32 && L'0' == 48);
static_assert(L'A' == 65 && L'a' == 97 && L'~' == 126);
static_assert(L"\0"[0] == 0 && "\0"[0] == 0);
