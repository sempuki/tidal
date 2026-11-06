// A private base: PD lvalue against Base lvalue is rejected.
struct Base { int v; };
struct PD : private Base { };
int main() { PD p{}; Base b{}; bool c = true; Base& r = c ? p : b; (void)r; return 0; }
