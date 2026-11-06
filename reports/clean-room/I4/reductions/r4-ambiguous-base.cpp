struct T { };
struct L : T { };
struct R : T { };
struct B : L, R { };
int main() { B b; T t; (void)(true ? b : t); return 0; }
