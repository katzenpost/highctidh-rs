/* The <endian.h> highctidh's naidne.h includes, for libcs such as newlib that lack it. */
#ifndef HIGHCTIDH_BARE_METAL_ENDIAN_H
#define HIGHCTIDH_BARE_METAL_ENDIAN_H
#if defined(__BYTE_ORDER__) && __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
#define htole32(x) __builtin_bswap32(x)
#define htole64(x) __builtin_bswap64(x)
#else
#define htole32(x) (x)
#define htole64(x) (x)
#endif
#define le32toh(x) htole32(x)
#define le64toh(x) htole64(x)
#endif
