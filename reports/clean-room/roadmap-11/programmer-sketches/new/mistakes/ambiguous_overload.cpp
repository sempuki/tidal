// Two overloads that a call reaches with equal conversions.
void f(long);
void f(unsigned);
int main() { f(1); return 0; }
