// tidal-rules: TRY-11
// tidal-expect: accept
// tidal-note: Compiles on every compiler. The call of a member function of a destroyed member, and the address of that member, are undefined, because the members are destroyed before the handler runs.
struct Part { int get() const { return 1; } };
struct S { Part p; S() try : p() { throw 0; } catch (int) { int v = p.get(); const Part* q = &p; (void)v; (void)q; } };
