namespace std { class type_info { public: virtual ~type_info(); bool operator==(const type_info&) const noexcept; const char* name() const noexcept; }; }
struct B { virtual ~B() = default; }; struct D : B {};
int main() { D d; B& b = d; return typeid(b) == typeid(D) ? 0 : 1; }
