// ICS-38 with LIST-6: a UTF-8 literal in braces initializes a char array.
struct Utf8 { Utf8(const char8_t*) {} };
int take(const char (&)[3]) { return 1; }
int take(Utf8) { return 2; }
int main() { return take({u8"ab"}) == 1 ? 0 : 1; }
