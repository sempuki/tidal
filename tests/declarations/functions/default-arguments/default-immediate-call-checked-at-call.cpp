// tidal-rules: DEFARG-10
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: consteval
// tidal-note: Q-0379. The default argument calls the consteval function width, which has no definition yet at the declaration of pad. The call is neither evaluated nor checked as a constant at the declaration. Each use in a call is checked there. Clang and GCC accept. EDG checks the call at the declaration and rejects it.
consteval int width();
int pad(int w = width()) { return w; }
consteval int width() { return 3; }
int main() { return pad() == 3 ? 0 : 1; }
