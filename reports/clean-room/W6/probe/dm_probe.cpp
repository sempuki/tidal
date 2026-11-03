struct Window { int width = 640; bool visible; int area = width * 2; };
#ifdef A
const Window cw;                 // CTOR-4: visible has no default
#endif
#ifdef B
struct P { int x(5); };          // CLS-9: parentheses
#endif
#ifdef C
constexpr Window cx{.width = 2}; // CE-21? visible becomes zero via LIST-8
static_assert(cx.area == 4);
#endif
#ifdef D
int f() { constexpr Window d; return d.width; }  // CX-1
#endif
int main() {}
