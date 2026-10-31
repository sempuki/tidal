// tidal-rules: TRY-17
// tidal-expect: reject
struct S { S() try { } catch (int) { return; } };
