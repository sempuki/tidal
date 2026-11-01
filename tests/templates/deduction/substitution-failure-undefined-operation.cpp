// tidal-rules: DED-24 CE-9
// tidal-expect: run
// tidal-note: The template argument divides by zero, so its evaluation fails. The failure is a substitution failure, and the other overload is chosen.
template <int N> struct Box {};
constexpr int divide(int a, int b) { return a / b; }
template <int B> int pick(Box<divide(1, B)>*) { return 1; }
template <int B> int pick(...) { return 2; }
int main() { return pick<1>(nullptr) == 1 && pick<0>(nullptr) == 2 ? 0 : 1; }
