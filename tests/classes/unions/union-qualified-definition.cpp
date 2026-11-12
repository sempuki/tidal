// tidal-rules: UNI-17
// tidal-expect: accept
// tidal-note: Front-end ledger row G182. A union declared in a namespace and a union declared in a class are each defined outside with a qualified name, one of them with final.
namespace store { union Cell; }
union store::Cell { int whole; char bytes[4]; };
struct Packet { union Field; };
union Packet::Field final { short half; long wide; };
constexpr store::Cell cell{7};
constexpr Packet::Field field{3};
static_assert(cell.whole == 7);
static_assert(field.half == 3);
static_assert(sizeof(Packet::Field) == sizeof(long));
