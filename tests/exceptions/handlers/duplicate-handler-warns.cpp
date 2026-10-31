// tidal-rules: TRY-4
// tidal-expect: reject
// tidal-flags: -Wall -Werror
void f() { try { } catch (int) { } catch (int) { } }
