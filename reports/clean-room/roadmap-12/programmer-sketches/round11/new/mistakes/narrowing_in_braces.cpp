// A narrowing conversion from double inside braces.
struct P { int x, y; };
int main() { double d = 2.5; P p{d, 1}; return p.x; }
