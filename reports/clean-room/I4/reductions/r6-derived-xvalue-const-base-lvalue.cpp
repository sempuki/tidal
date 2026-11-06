struct T { };
struct D : T { };
D d;
const T t{};
int main() { (void)(true ? static_cast<D&&>(d) : t); return 0; }
