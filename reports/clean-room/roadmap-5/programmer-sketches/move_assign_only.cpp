struct P { int x; P& operator=(P&& o) { x = o.x; return *this; } P(int v) : x(v) {} };
int main() { P a{1}; a = P{2}; return a.x == 2 ? 0 : 1; }
