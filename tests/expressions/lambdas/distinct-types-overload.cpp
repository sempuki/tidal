// tidal-rules: LAM-2
// tidal-expect: run
// tidal-note: Two lambdas with the same text have different closure types, so two functions that take them are two overloads.
auto a = [](int x) { return x; };
auto b = [](int x) { return x; };
int pick(decltype(a)) { return 1; }
int pick(decltype(b)) { return 2; }
int main() { return pick(a) == 1 && pick(b) == 2 ? 0 : 1; }
