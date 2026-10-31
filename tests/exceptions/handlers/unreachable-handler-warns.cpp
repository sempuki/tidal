// tidal-rules: TRY-4
// tidal-expect: reject
// tidal-flags: -Wall -Werror
struct B {}; struct D : B {};
void f() { try { } catch (B&) { } catch (D&) { } }
