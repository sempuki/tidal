template <int N> struct Pt { int x[N]; };
template <Pt<1> P> int get() { return P.x[0]; }
int main() { return get<Pt<1>{{3}}>() - 3; }
