// tidal-rules: DEL-1 OVL-37
// tidal-expect: reject
// tidal-error-clang: call to deleted function .put.
// tidal-error-gcc: use of deleted function
// tidal-error-edg: function .put\(int\). .* cannot be referenced -- it is a deleted function
// tidal-note: The deleted put(int) is viable and wins. put(long) does not take its place.
void put(int) = delete;
void put(long);
void use() { put(1); }
