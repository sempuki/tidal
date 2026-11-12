constexpr char8_t s[] = u8"hé";
static_assert(sizeof(s) == 4);
static_assert(s[1] == 0xC3 && s[2] == 0xA9);
int main() { char8_t c = u8'a'; return c - 97; }
