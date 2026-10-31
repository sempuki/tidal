// tidal-rules: CE-11
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Gap review 2026-10-31. Q-0078. EDG converts the condition to bool and permits narrowing.
struct S { explicit(2) S(int); };
