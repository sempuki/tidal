// tidal-rules: DEFARG-10
// tidal-expect: reject
// tidal-error: consteval function
// tidal-note: Q-0379. The default argument passes the variable limit, which is not a constant, to a consteval function, and main calls pad with the default. The call that uses the default checks the immediate call, and limit is not a constant there. Clang and GCC reject at the call. EDG rejects at the declaration.
int limit = 2;
consteval int clamp(int v) { return v; }
int pad(int w = clamp(limit)) { return w; }
int main() { return pad() == 2 ? 0 : 1; }
