consteval int sq(int x) { return x * x; }
template <int N> struct Buf { char b[N]; };
int main() { Buf<sq(4)> b{}; return sizeof b == 16 ? 0 : 1; }
