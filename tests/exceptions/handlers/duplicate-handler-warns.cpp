// tidal-rules: TRY-4
// tidal-expect: reject
// tidal-error-clang: will be caught by earlier handler
// tidal-error-gcc: will be caught by earlier handler
// tidal-flags: -Wall -Werror
void f() { try { } catch (int) { } catch (int) { } }
