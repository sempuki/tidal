struct [[nodiscard]] R { int v; };
constexpr R f() noexcept { return {1}; }
int main() { f(); return 0; }
