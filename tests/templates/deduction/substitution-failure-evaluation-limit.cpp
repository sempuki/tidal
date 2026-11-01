// tidal-rules: DED-24 CE-10
// tidal-expect: run
// tidal-note: A recursion depth of 100000 exceeds the limit of every matrix compiler (CE-10). The failed evaluation is a substitution failure, so the choice of overload depends on the limit.
template <int N> struct Box {};
constexpr int depth(int n) { return n == 0 ? 0 : 1 + depth(n - 1); }
template <int D> int pick(Box<depth(D)>*) { return 1; }
template <int D> int pick(...) { return 2; }
int main() { return pick<10>(nullptr) == 1 && pick<100000>(nullptr) == 2 ? 0 : 1; }
