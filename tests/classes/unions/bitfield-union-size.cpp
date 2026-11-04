// tidal-rules: BF-9 UNI-1
// tidal-expect: accept
// tidal-note: x86-64 and -m32 alike. A bit-field in a union starts at byte 0 and takes the bytes that its width needs. The union gets the alignment of the bit-field's type, or of the larger integer type for a wide bit-field.
union Nibble { unsigned x : 4; };
static_assert(sizeof(Nibble) == 4 && alignof(Nibble) == 4);
union Twelve { unsigned char c : 12; };
static_assert(sizeof(Twelve) == 2 && alignof(Twelve) == 1);
union Forty { int x : 40; };
static_assert(sizeof(Forty) == 8 && alignof(Forty) == 4);
union Small { short x : 3; char y[3]; };
static_assert(sizeof(Small) == 4 && alignof(Small) == 2);
