// tidal-rules: CMP-7
// tidal-expect: accept
struct Record { int values[2]; int tail; };
Record record;
static_assert(&record.values[1] < &record.tail);
static_assert(record.values + 2 <= &record.tail);
static_assert(&record.values[0] < &record.values[1]);
