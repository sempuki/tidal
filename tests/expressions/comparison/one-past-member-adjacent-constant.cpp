// tidal-rules: CMP-20
// tidal-expect: accept
struct Record { int values[2]; int tail; };
Record record;
constexpr bool same_place = record.values + 2 == &record.tail;
static_assert(record.values + 2 <= &record.tail);
