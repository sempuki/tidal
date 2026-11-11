template <class... F> struct overloaded : F... { using F::operator()...; };
int main() { overloaded o{[](int) { return 1; }, [](double) { return 2; }}; return o(1) + o(1.0) == 3 ? 0 : 1; }
