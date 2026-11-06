struct P { int v; P(int x) : v(x) {} };
int main() { P a[] = {1}; return a[0].v - 1; }
