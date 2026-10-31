// tidal-rules: SA-2
// tidal-expect: run
int main() { static_assert(true); int x = 0; static_assert(sizeof(x) == sizeof(int)); return x; }
