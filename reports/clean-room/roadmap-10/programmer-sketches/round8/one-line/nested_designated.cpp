struct In { int a = 1, b = 2; };
struct Out { In in; int c = 3; };
int main() { Out o{.in = {.b = 5}, .c = 4}; return o.in.a + o.in.b + o.c == 10 ? 0 : 1; }
