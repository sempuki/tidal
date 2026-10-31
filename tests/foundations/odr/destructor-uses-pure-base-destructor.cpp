// tidal-rules: ODR-4 ODR-8
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .Base::~Base\(\)
// tidal-error-gcc: undefined reference to .Base::~Base\(\)
struct Base { virtual ~Base() = 0; };
struct Derived : Base { };
int main() { Derived d; return 0; }
