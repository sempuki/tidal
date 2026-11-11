struct Pt { int x, y; };
template <Pt P> int sum() { return P.x + P.y; }
int main() { return sum<Pt{1, 2}>() - 3; }
