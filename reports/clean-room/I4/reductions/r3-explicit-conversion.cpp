struct E { explicit operator int() { return 0; } };
int main() { E e; return true ? e : 0; }
