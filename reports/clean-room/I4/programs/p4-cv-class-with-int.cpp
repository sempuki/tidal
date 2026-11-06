// const A and volatile A lvalues, A converts to int (COND-P2 against COND-11).
struct A { operator int() const volatile { return 3; } };
const A ca{};
volatile A va{};
int main() { bool c = true; int x = c ? ca : va; return x == 3 ? 0 : 1; }
