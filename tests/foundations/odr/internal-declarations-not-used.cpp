// tidal-rules: ODR-9
// tidal-expect: run
static int hidden();
namespace { extern int also_hidden; }
int main() { return sizeof(hidden()) == sizeof(also_hidden) ? 0 : 1; }
