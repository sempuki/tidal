struct D { ~D() {} };
struct S { int a; float b; };
static_assert(__is_trivially_destructible(int));
static_assert(!__is_trivially_destructible(D));
static_assert(__is_trivially_copyable(S));
static_assert(__is_standard_layout(S));
constexpr float f = 1.0f;
static_assert(__builtin_bit_cast(unsigned, f) == 0x3f800000u);
int x = 1;
int* p = __builtin_launder(&x);
static_assert(__STDCPP_DEFAULT_NEW_ALIGNMENT__ == 16);
int main() {}
