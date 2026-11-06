// An ambiguous base: Bottom lvalue against Top lvalue is rejected (CONV-17).
struct Top { int v; };
struct L : Top { };
struct R : Top { };
struct Bottom : L, R { };
int main() { Bottom bo{}; Top t{}; bool c = true; Top& r = c ? bo : t; return 0; }
