// tidal-rules: DEFARG-10
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: consteval
// tidal-note: Q-0379. The default argument passes the variable limit, which is not a constant, to a consteval function. No call uses the default, so the immediate call is never checked. Clang and GCC accept. EDG checks the call at the declaration and rejects it.
int limit = 2;
consteval int clamp(int v) { return v; }
int pad(int w = clamp(limit)) { return w; }
