// tidal-rules: MPTR-4
// tidal-expect: run
struct B { int x; };
struct D : B {};
int main() { int B::* pb = nullptr; int D::* pd = pb; return pd == nullptr ? 0 : 1; }
