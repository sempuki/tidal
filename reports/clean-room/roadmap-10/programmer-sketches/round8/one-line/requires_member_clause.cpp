template <int N> struct Buf { int f() requires (N > 0) { return N; } int f() requires (N == 0) { return -1; } };
int main() { return Buf<3>{}.f() + Buf<0>{}.f() == 2 ? 0 : 1; }
