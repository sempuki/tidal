// tidal-rules: TRY-11
// tidal-expect: accept
// tidal-note: Compiles on every compiler. Using the member is undefined, because the members are destroyed before the handler runs.
struct S { int m; S() try : m(1) { } catch (int) { int v = m; (void)v; } };
