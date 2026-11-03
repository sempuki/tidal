// tidal-rules: ENUM-13
// tidal-expect: accept
// tidal-note: Q-0114. The enumerator before has the value of the largest unsigned int. The next value needs at least 33 bits, so the next enumerator has some type that holds it.
enum Grows { first = 0xffffffff, second, holds = second > first && sizeof(second) >= 8 ? 1 : 0 };
static_assert(holds == 1);
