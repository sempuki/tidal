// tidal-rules: TRY-3
// tidal-expect: reject
// tidal-error-clang: catch-all handler must come last
// tidal-error-gcc: handler must be the last handler
// tidal-error-edg: masked by default handler
void f() { try { } catch (...) { } catch (int) { } }
