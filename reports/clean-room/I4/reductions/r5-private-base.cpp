struct T { };
struct P : private T { };
int main() { P p; T t; (void)(true ? p : t); return 0; }
