// tidal-rules: DEFARG-4
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: parameter is not allowed
// tidal-note: Gap review 2026-10-30. Q-0038. EDG rejects a parameter in an unevaluated default argument.
int f(int a, int b = sizeof(a));
