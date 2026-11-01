// tidal-rules: CMP-6
// tidal-expect: accept
int list[2];
static_assert(list < &list[1]);
static_assert(list == &list[0]);
static_assert(&list[1] > list);
static_assert(list != nullptr);
