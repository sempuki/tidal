// tidal-rules: TYN-1
// tidal-expect: run
struct S { using size_type = unsigned; typedef int value_type; size_type n = 3; };
int main() { S::size_type x = S{}.n; S::value_type v = -1; return x == 3 && v == -1 ? 0 : 1; }
