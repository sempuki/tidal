extern "C" { using cb = int(int); }
int f(cb* p, int v) { return p(v); }
int g(int v) { return v; }
int main() { return f(g, 0); }
