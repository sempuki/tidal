template <class T> const T& max_of(const T& a, const T& b) { return a < b ? b : a; }
struct W { int v; };
bool operator<(const W& a, const W& b) { return a.v < b.v; }
int main() { const W& m = max_of(W{1}, W{2}); return m.v == 2 ? 0 : 1; }
