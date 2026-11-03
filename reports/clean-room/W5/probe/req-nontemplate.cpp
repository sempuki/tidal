constexpr int twice(int v) { return 2 * v; }
int twice(double) = delete;
static_assert(requires { twice(1); });
static_assert(!requires { twice(1.5); });
int main() {}
