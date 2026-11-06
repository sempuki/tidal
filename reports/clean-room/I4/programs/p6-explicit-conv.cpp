// An explicit conversion to int does not take part (CVF-11): rejected.
struct E { explicit operator int() const { return 1; } };
int main() { E e; bool c = true; int x = c ? e : 2; return x; }
