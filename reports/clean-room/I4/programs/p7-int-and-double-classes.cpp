// Two classes, one to int and one to double: COND-11 gives double.
struct I { operator int() const { return 1; } };
struct D { operator double() const { return 2.5; } };
int main() { I i; D d; double x = false ? i : d; double y = true ? i : d; return x == 2.5 && y == 1.0 ? 0 : 1; }
