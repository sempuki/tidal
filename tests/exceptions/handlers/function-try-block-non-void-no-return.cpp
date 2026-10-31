// tidal-rules: TRY-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Clang warns under -Wall; GCC gives no warning. EDG rejects it without flags.
// tidal-flags: -Wall -Werror
int f() try { return 1; } catch (int) { }
int main() { return f(); }
