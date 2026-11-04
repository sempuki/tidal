// tidal-rules: CE-24
// tidal-expect: run
// tidal-note: Q-0192. The default constructor runs and the class has no scalar part, so the call is a constant initializer.
struct Empty {};
constinit Empty empty;
int main() { return 0; }
