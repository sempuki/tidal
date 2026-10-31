// tidal-rules: TRY-4
// tidal-expect: accept
// tidal-flags: -Wall -Werror
struct B {}; struct D : B {};
void f() { try { } catch (D&) { } catch (B&) { } }
