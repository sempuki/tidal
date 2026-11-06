struct alignas(16) V { float x, y, z, w; };
static_assert(alignof(V) == 16 && sizeof(V) == 16);
int main() { V a[2]; return reinterpret_cast<unsigned long>(&a[1]) % 16 == 0 ? 0 : 1; }
