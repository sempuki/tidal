// Aggregate initialization with parentheses (C++20).
struct P { int x; int y = 5; };
int main() { P p(2); P q(1, 2); return p.x + p.y + q.x + q.y - 10; }
