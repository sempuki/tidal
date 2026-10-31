// tidal-rules: ODR-5 ODR-8
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .Item::~Item\(\)
// tidal-error-gcc: undefined reference to .Item::~Item\(\)
struct Item { int v = 0; ~Item(); };
int main() { Item* item = new Item; delete item; return 0; }
