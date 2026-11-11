struct S { int v; int twice() const { return 2 * v; } };
template <class F, class O> auto invoke(F f, const O& o) { return (o.*f)(); }
int main() { S s{4}; return invoke(&S::twice, s) == 8 ? 0 : 1; }
