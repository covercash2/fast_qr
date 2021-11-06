//! Enum containing all possible QRCode versions

use super::encode::Mode;
use super::ECL;

#[cfg(test)]
mod t_versionformat;

#[derive(Clone, Copy)]
/// Enum containing all possible QRCode versions
pub enum Version {
    V1 = 0,
    V2 = 1,
    V3 = 2,
    V4 = 3,
    V5 = 4,
    V6 = 5,
    V7 = 6,
    V8 = 7,
    V9 = 8,
    V10 = 9,
    V11 = 10,
    V12 = 11,
    V13 = 12,
    V14 = 13,
    V15 = 14,
    V16 = 15,
    V17 = 16,
    V18 = 17,
    V19 = 18,
    V20 = 19,
    V21 = 20,
    V22 = 21,
    V23 = 22,
    V24 = 23,
    V25 = 24,
    V26 = 25,
    V27 = 26,
    V28 = 27,
    V29 = 28,
    V30 = 29,
    V31 = 30,
    V32 = 31,
    V33 = 32,
    V34 = 33,
    V35 = 34,
    V36 = 35,
    V37 = 36,
    V38 = 37,
    V39 = 38,
    V40 = 39,
}

impl Version {
    /// Computes the **best version** according to `mode`, `ecl` and `len``
    ///
    /// # Example
    /// ```txt
    /// let input = b"Hello, world!";
    /// let ecl = ecl::ECL::H;
    /// let mode = encode::best_encoding(input);
    /// let version = match version::Version::get(mode, ecl, input.len()) {
    ///     Some(version) => version,
    ///     None => return (),
    /// };
    /// ```
    pub const fn get(mode: Mode, ecl: ECL, len: usize) -> Option<Self> {
        use Version::*;

        match mode {
            Mode::Numeric => match ecl {
                ECL::L => match len {
                    0..=41 => Some(V1),
                    0..=77 => Some(V2),
                    0..=127 => Some(V3),
                    0..=187 => Some(V4),
                    0..=255 => Some(V5),
                    0..=322 => Some(V6),
                    0..=370 => Some(V7),
                    0..=461 => Some(V8),
                    0..=552 => Some(V9),
                    0..=652 => Some(V10),
                    0..=772 => Some(V11),
                    0..=883 => Some(V12),
                    0..=1022 => Some(V13),
                    0..=1101 => Some(V14),
                    0..=1250 => Some(V15),
                    0..=1408 => Some(V16),
                    0..=1548 => Some(V17),
                    0..=1725 => Some(V18),
                    0..=1903 => Some(V19),
                    0..=2061 => Some(V20),
                    0..=2232 => Some(V21),
                    0..=2409 => Some(V22),
                    0..=2620 => Some(V23),
                    0..=2812 => Some(V24),
                    0..=3057 => Some(V25),
                    0..=3283 => Some(V26),
                    0..=3517 => Some(V27),
                    0..=3669 => Some(V28),
                    0..=3909 => Some(V29),
                    0..=4158 => Some(V30),
                    0..=4417 => Some(V31),
                    0..=4686 => Some(V32),
                    0..=4965 => Some(V33),
                    0..=5253 => Some(V34),
                    0..=5529 => Some(V35),
                    0..=5836 => Some(V36),
                    0..=6153 => Some(V37),
                    0..=6479 => Some(V38),
                    0..=6743 => Some(V39),
                    0..=7089 => Some(V40),
                    _ => None,
                },
                ECL::M => match len {
                    0..=34 => Some(V1),
                    0..=63 => Some(V2),
                    0..=101 => Some(V3),
                    0..=149 => Some(V4),
                    0..=202 => Some(V5),
                    0..=255 => Some(V6),
                    0..=293 => Some(V7),
                    0..=365 => Some(V8),
                    0..=432 => Some(V9),
                    0..=513 => Some(V10),
                    0..=604 => Some(V11),
                    0..=691 => Some(V12),
                    0..=796 => Some(V13),
                    0..=871 => Some(V14),
                    0..=991 => Some(V15),
                    0..=1082 => Some(V16),
                    0..=1212 => Some(V17),
                    0..=1346 => Some(V18),
                    0..=1500 => Some(V19),
                    0..=1600 => Some(V20),
                    0..=1708 => Some(V21),
                    0..=1872 => Some(V22),
                    0..=2059 => Some(V23),
                    0..=2188 => Some(V24),
                    0..=2395 => Some(V25),
                    0..=2544 => Some(V26),
                    0..=2701 => Some(V27),
                    0..=2857 => Some(V28),
                    0..=3035 => Some(V29),
                    0..=3289 => Some(V30),
                    0..=3486 => Some(V31),
                    0..=3693 => Some(V32),
                    0..=3909 => Some(V33),
                    0..=4134 => Some(V34),
                    0..=4343 => Some(V35),
                    0..=4588 => Some(V36),
                    0..=4775 => Some(V37),
                    0..=5039 => Some(V38),
                    0..=5313 => Some(V39),
                    0..=5596 => Some(V40),
                    _ => None,
                },
                ECL::Q => match len {
                    0..=27 => Some(V1),
                    0..=48 => Some(V2),
                    0..=77 => Some(V3),
                    0..=111 => Some(V4),
                    0..=144 => Some(V5),
                    0..=178 => Some(V6),
                    0..=207 => Some(V7),
                    0..=259 => Some(V8),
                    0..=312 => Some(V9),
                    0..=364 => Some(V10),
                    0..=427 => Some(V11),
                    0..=489 => Some(V12),
                    0..=580 => Some(V13),
                    0..=621 => Some(V14),
                    0..=703 => Some(V15),
                    0..=775 => Some(V16),
                    0..=876 => Some(V17),
                    0..=948 => Some(V18),
                    0..=1063 => Some(V19),
                    0..=1159 => Some(V20),
                    0..=1224 => Some(V21),
                    0..=1358 => Some(V22),
                    0..=1468 => Some(V23),
                    0..=1588 => Some(V24),
                    0..=1718 => Some(V25),
                    0..=1804 => Some(V26),
                    0..=1933 => Some(V27),
                    0..=2085 => Some(V28),
                    0..=2181 => Some(V29),
                    0..=2358 => Some(V30),
                    0..=2473 => Some(V31),
                    0..=2670 => Some(V32),
                    0..=2805 => Some(V33),
                    0..=2949 => Some(V34),
                    0..=3081 => Some(V35),
                    0..=3244 => Some(V36),
                    0..=3417 => Some(V37),
                    0..=3599 => Some(V38),
                    0..=3791 => Some(V39),
                    0..=3993 => Some(V40),
                    _ => None,
                },
                ECL::H => match len {
                    0..=17 => Some(V1),
                    0..=34 => Some(V2),
                    0..=58 => Some(V3),
                    0..=82 => Some(V4),
                    0..=106 => Some(V5),
                    0..=139 => Some(V6),
                    0..=154 => Some(V7),
                    0..=202 => Some(V8),
                    0..=235 => Some(V9),
                    0..=288 => Some(V10),
                    0..=331 => Some(V11),
                    0..=374 => Some(V12),
                    0..=427 => Some(V13),
                    0..=468 => Some(V14),
                    0..=530 => Some(V15),
                    0..=602 => Some(V16),
                    0..=674 => Some(V17),
                    0..=746 => Some(V18),
                    0..=813 => Some(V19),
                    0..=919 => Some(V20),
                    0..=969 => Some(V21),
                    0..=1056 => Some(V22),
                    0..=1108 => Some(V23),
                    0..=1228 => Some(V24),
                    0..=1286 => Some(V25),
                    0..=1425 => Some(V26),
                    0..=1501 => Some(V27),
                    0..=1581 => Some(V28),
                    0..=1677 => Some(V29),
                    0..=1782 => Some(V30),
                    0..=1897 => Some(V31),
                    0..=2022 => Some(V32),
                    0..=2157 => Some(V33),
                    0..=2301 => Some(V34),
                    0..=2361 => Some(V35),
                    0..=2524 => Some(V36),
                    0..=2625 => Some(V37),
                    0..=2735 => Some(V38),
                    0..=2927 => Some(V39),
                    0..=3057 => Some(V40),
                    _ => None,
                },
            },
            Mode::Alphanumeric => match ecl {
                ECL::L => match len {
                    0..=25 => Some(V1),
                    0..=47 => Some(V2),
                    0..=77 => Some(V3),
                    0..=114 => Some(V4),
                    0..=154 => Some(V5),
                    0..=195 => Some(V6),
                    0..=224 => Some(V7),
                    0..=279 => Some(V8),
                    0..=335 => Some(V9),
                    0..=395 => Some(V10),
                    0..=468 => Some(V11),
                    0..=535 => Some(V12),
                    0..=619 => Some(V13),
                    0..=667 => Some(V14),
                    0..=758 => Some(V15),
                    0..=854 => Some(V16),
                    0..=938 => Some(V17),
                    0..=1046 => Some(V18),
                    0..=1153 => Some(V19),
                    0..=1249 => Some(V20),
                    0..=1352 => Some(V21),
                    0..=1460 => Some(V22),
                    0..=1588 => Some(V23),
                    0..=1704 => Some(V24),
                    0..=1853 => Some(V25),
                    0..=1990 => Some(V26),
                    0..=2132 => Some(V27),
                    0..=2223 => Some(V28),
                    0..=2369 => Some(V29),
                    0..=2520 => Some(V30),
                    0..=2677 => Some(V31),
                    0..=2840 => Some(V32),
                    0..=3009 => Some(V33),
                    0..=3183 => Some(V34),
                    0..=3351 => Some(V35),
                    0..=3537 => Some(V36),
                    0..=3729 => Some(V37),
                    0..=3927 => Some(V38),
                    0..=4087 => Some(V39),
                    0..=4296 => Some(V40),
                    _ => None,
                },
                ECL::M => match len {
                    0..=20 => Some(V1),
                    0..=38 => Some(V2),
                    0..=61 => Some(V3),
                    0..=90 => Some(V4),
                    0..=122 => Some(V5),
                    0..=154 => Some(V6),
                    0..=178 => Some(V7),
                    0..=221 => Some(V8),
                    0..=262 => Some(V9),
                    0..=311 => Some(V10),
                    0..=366 => Some(V11),
                    0..=419 => Some(V12),
                    0..=483 => Some(V13),
                    0..=528 => Some(V14),
                    0..=600 => Some(V15),
                    0..=656 => Some(V16),
                    0..=734 => Some(V17),
                    0..=816 => Some(V18),
                    0..=909 => Some(V19),
                    0..=970 => Some(V20),
                    0..=1035 => Some(V21),
                    0..=1134 => Some(V22),
                    0..=1248 => Some(V23),
                    0..=1326 => Some(V24),
                    0..=1451 => Some(V25),
                    0..=1542 => Some(V26),
                    0..=1637 => Some(V27),
                    0..=1732 => Some(V28),
                    0..=1839 => Some(V29),
                    0..=1994 => Some(V30),
                    0..=2113 => Some(V31),
                    0..=2238 => Some(V32),
                    0..=2369 => Some(V33),
                    0..=2506 => Some(V34),
                    0..=2632 => Some(V35),
                    0..=2780 => Some(V36),
                    0..=2894 => Some(V37),
                    0..=3054 => Some(V38),
                    0..=3220 => Some(V39),
                    0..=3391 => Some(V40),
                    _ => None,
                },
                ECL::Q => match len {
                    0..=16 => Some(V1),
                    0..=29 => Some(V2),
                    0..=47 => Some(V3),
                    0..=67 => Some(V4),
                    0..=87 => Some(V5),
                    0..=108 => Some(V6),
                    0..=125 => Some(V7),
                    0..=157 => Some(V8),
                    0..=189 => Some(V9),
                    0..=221 => Some(V10),
                    0..=259 => Some(V11),
                    0..=296 => Some(V12),
                    0..=352 => Some(V13),
                    0..=376 => Some(V14),
                    0..=426 => Some(V15),
                    0..=470 => Some(V16),
                    0..=531 => Some(V17),
                    0..=574 => Some(V18),
                    0..=644 => Some(V19),
                    0..=702 => Some(V20),
                    0..=742 => Some(V21),
                    0..=823 => Some(V22),
                    0..=890 => Some(V23),
                    0..=963 => Some(V24),
                    0..=1041 => Some(V25),
                    0..=1094 => Some(V26),
                    0..=1172 => Some(V27),
                    0..=1263 => Some(V28),
                    0..=1322 => Some(V29),
                    0..=1429 => Some(V30),
                    0..=1499 => Some(V31),
                    0..=1618 => Some(V32),
                    0..=1700 => Some(V33),
                    0..=1787 => Some(V34),
                    0..=1867 => Some(V35),
                    0..=1966 => Some(V36),
                    0..=2071 => Some(V37),
                    0..=2181 => Some(V38),
                    0..=2298 => Some(V39),
                    0..=2420 => Some(V40),
                    _ => None,
                },
                ECL::H => match len {
                    0..=10 => Some(V1),
                    0..=20 => Some(V2),
                    0..=35 => Some(V3),
                    0..=50 => Some(V4),
                    0..=64 => Some(V5),
                    0..=84 => Some(V6),
                    0..=93 => Some(V7),
                    0..=122 => Some(V8),
                    0..=143 => Some(V9),
                    0..=174 => Some(V10),
                    0..=200 => Some(V11),
                    0..=227 => Some(V12),
                    0..=259 => Some(V13),
                    0..=283 => Some(V14),
                    0..=321 => Some(V15),
                    0..=365 => Some(V16),
                    0..=408 => Some(V17),
                    0..=452 => Some(V18),
                    0..=493 => Some(V19),
                    0..=557 => Some(V20),
                    0..=587 => Some(V21),
                    0..=640 => Some(V22),
                    0..=672 => Some(V23),
                    0..=744 => Some(V24),
                    0..=779 => Some(V25),
                    0..=864 => Some(V26),
                    0..=910 => Some(V27),
                    0..=958 => Some(V28),
                    0..=1016 => Some(V29),
                    0..=1080 => Some(V30),
                    0..=1150 => Some(V31),
                    0..=1226 => Some(V32),
                    0..=1307 => Some(V33),
                    0..=1394 => Some(V34),
                    0..=1431 => Some(V35),
                    0..=1530 => Some(V36),
                    0..=1591 => Some(V37),
                    0..=1658 => Some(V38),
                    0..=1774 => Some(V39),
                    0..=1852 => Some(V40),
                    _ => None,
                },
            },
            Mode::Byte => match ecl {
                ECL::L => match len {
                    0..=17 => Some(V1),
                    0..=32 => Some(V2),
                    0..=53 => Some(V3),
                    0..=78 => Some(V4),
                    0..=106 => Some(V5),
                    0..=134 => Some(V6),
                    0..=154 => Some(V7),
                    0..=192 => Some(V8),
                    0..=230 => Some(V9),
                    0..=271 => Some(V10),
                    0..=321 => Some(V11),
                    0..=367 => Some(V12),
                    0..=425 => Some(V13),
                    0..=458 => Some(V14),
                    0..=520 => Some(V15),
                    0..=586 => Some(V16),
                    0..=644 => Some(V17),
                    0..=718 => Some(V18),
                    0..=792 => Some(V19),
                    0..=858 => Some(V20),
                    0..=929 => Some(V21),
                    0..=1003 => Some(V22),
                    0..=1091 => Some(V23),
                    0..=1171 => Some(V24),
                    0..=1273 => Some(V25),
                    0..=1367 => Some(V26),
                    0..=1465 => Some(V27),
                    0..=1528 => Some(V28),
                    0..=1628 => Some(V29),
                    0..=1732 => Some(V30),
                    0..=1840 => Some(V31),
                    0..=1952 => Some(V32),
                    0..=2068 => Some(V33),
                    0..=2188 => Some(V34),
                    0..=2303 => Some(V35),
                    0..=2431 => Some(V36),
                    0..=2563 => Some(V37),
                    0..=2699 => Some(V38),
                    0..=2809 => Some(V39),
                    0..=2953 => Some(V40),
                    _ => None,
                },
                ECL::M => match len {
                    0..=14 => Some(V1),
                    0..=26 => Some(V2),
                    0..=42 => Some(V3),
                    0..=62 => Some(V4),
                    0..=84 => Some(V5),
                    0..=106 => Some(V6),
                    0..=122 => Some(V7),
                    0..=152 => Some(V8),
                    0..=180 => Some(V9),
                    0..=213 => Some(V10),
                    0..=251 => Some(V11),
                    0..=287 => Some(V12),
                    0..=331 => Some(V13),
                    0..=362 => Some(V14),
                    0..=412 => Some(V15),
                    0..=450 => Some(V16),
                    0..=504 => Some(V17),
                    0..=560 => Some(V18),
                    0..=624 => Some(V19),
                    0..=666 => Some(V20),
                    0..=711 => Some(V21),
                    0..=779 => Some(V22),
                    0..=857 => Some(V23),
                    0..=911 => Some(V24),
                    0..=997 => Some(V25),
                    0..=1059 => Some(V26),
                    0..=1125 => Some(V27),
                    0..=1190 => Some(V28),
                    0..=1264 => Some(V29),
                    0..=1370 => Some(V30),
                    0..=1452 => Some(V31),
                    0..=1538 => Some(V32),
                    0..=1628 => Some(V33),
                    0..=1722 => Some(V34),
                    0..=1809 => Some(V35),
                    0..=1911 => Some(V36),
                    0..=1989 => Some(V37),
                    0..=2099 => Some(V38),
                    0..=2213 => Some(V39),
                    0..=2331 => Some(V40),
                    _ => None,
                },
                ECL::Q => match len {
                    0..=11 => Some(V1),
                    0..=20 => Some(V2),
                    0..=32 => Some(V3),
                    0..=46 => Some(V4),
                    0..=60 => Some(V5),
                    0..=74 => Some(V6),
                    0..=86 => Some(V7),
                    0..=108 => Some(V8),
                    0..=130 => Some(V9),
                    0..=151 => Some(V10),
                    0..=177 => Some(V11),
                    0..=203 => Some(V12),
                    0..=241 => Some(V13),
                    0..=258 => Some(V14),
                    0..=292 => Some(V15),
                    0..=322 => Some(V16),
                    0..=364 => Some(V17),
                    0..=394 => Some(V18),
                    0..=442 => Some(V19),
                    0..=482 => Some(V20),
                    0..=509 => Some(V21),
                    0..=565 => Some(V22),
                    0..=611 => Some(V23),
                    0..=661 => Some(V24),
                    0..=715 => Some(V25),
                    0..=751 => Some(V26),
                    0..=805 => Some(V27),
                    0..=868 => Some(V28),
                    0..=908 => Some(V29),
                    0..=982 => Some(V30),
                    0..=1030 => Some(V31),
                    0..=1112 => Some(V32),
                    0..=1168 => Some(V33),
                    0..=1228 => Some(V34),
                    0..=1283 => Some(V35),
                    0..=1351 => Some(V36),
                    0..=1423 => Some(V37),
                    0..=1499 => Some(V38),
                    0..=1579 => Some(V39),
                    0..=1663 => Some(V40),
                    _ => None,
                },
                ECL::H => match len {
                    0..=7 => Some(V1),
                    0..=14 => Some(V2),
                    0..=24 => Some(V3),
                    0..=34 => Some(V4),
                    0..=44 => Some(V5),
                    0..=58 => Some(V6),
                    0..=64 => Some(V7),
                    0..=84 => Some(V8),
                    0..=98 => Some(V9),
                    0..=119 => Some(V10),
                    0..=137 => Some(V11),
                    0..=155 => Some(V12),
                    0..=177 => Some(V13),
                    0..=194 => Some(V14),
                    0..=220 => Some(V15),
                    0..=250 => Some(V16),
                    0..=280 => Some(V17),
                    0..=310 => Some(V18),
                    0..=338 => Some(V19),
                    0..=382 => Some(V20),
                    0..=403 => Some(V21),
                    0..=439 => Some(V22),
                    0..=461 => Some(V23),
                    0..=511 => Some(V24),
                    0..=535 => Some(V25),
                    0..=593 => Some(V26),
                    0..=625 => Some(V27),
                    0..=658 => Some(V28),
                    0..=698 => Some(V29),
                    0..=742 => Some(V30),
                    0..=790 => Some(V31),
                    0..=842 => Some(V32),
                    0..=898 => Some(V33),
                    0..=958 => Some(V34),
                    0..=983 => Some(V35),
                    0..=1051 => Some(V36),
                    0..=1093 => Some(V37),
                    0..=1139 => Some(V38),
                    0..=1219 => Some(V39),
                    0..=1273 => Some(V40),
                    _ => None,
                },
            },
        }
    }

    /// Returns QRCode **missing padding bits count** at the very end
    pub const fn missing_bits(&self) -> usize {
        use Version::*;

        match self {
            V1 | V7 | V8 | V9 | V10 | V11 | V12 | V13 | V35 | V36 | V37 | V38 | V39 | V40 => 0,
            V14 | V15 | V16 | V17 | V18 | V19 | V20 | V28 | V29 | V30 | V31 | V32 | V33 | V34 => 3,
            V21 | V22 | V23 | V24 | V25 | V26 | V27 => 4,
            V2 | V3 | V4 | V5 | V6 => 7,
        }
    }

    /// Returns the **max bytes** that can contain a QRCode for a specified version
    pub const fn max_bytes(&self) -> usize {
        const MAX_BYTES: [usize; 40] = [
            26, 44, 70, 100, 134, 172, 196, 242, 292, 346, 404, 466, 532, 581, 655, 733, 815, 901,
            991, 1085, 1156, 1258, 1364, 1474, 1588, 1706, 1828, 1921, 2051, 2185, 2323, 2465,
            2611, 2761, 2876, 3034, 3196, 3362, 3532, 3706,
        ];

        return MAX_BYTES[*self as usize];
    }

    /// Returns the **version information** we need to put for QRCodes larger or equal to version 7
    pub const fn information(&self) -> u32 {
        const VERSION_INFORMATION: [u32; 40] = [
            0,
            0,
            0,
            0,
            0,
            0,
            0b000111110010010100,
            0b001000010110111100,
            0b001001101010011001,
            0b001010010011010011,
            0b001011101111110110,
            0b001100011101100010,
            0b001101100001000111,
            0b001110011000001101,
            0b001111100100101000,
            0b010000101101111000,
            0b010001010001011101,
            0b010010101000010111,
            0b010011010100110010,
            0b010100100110100110,
            0b010101011010000011,
            0b010110100011001001,
            0b010111011111101100,
            0b011000111011000100,
            0b011001000111100001,
            0b011010111110101011,
            0b011011000010001110,
            0b011100110000011010,
            0b011101001100111111,
            0b011110110101110101,
            0b011111001001010000,
            0b100000100111010101,
            0b100001011011110000,
            0b100010100010111010,
            0b100011011110011111,
            0b100100101100001011,
            0b100101010000101110,
            0b100110101001100100,
            0b100111010101000001,
            0b101000110001101001,
        ];

        return VERSION_INFORMATION[*self as usize];
    }

    /// Returns **alignments** positions
    pub const fn alignment_patterns_grid(&self) -> &'static [usize] {
        const ALIGNMENT_PATTERNS_GRID: [&'static [usize]; 40] = [
            &[],
            &[6, 18],
            &[6, 22],
            &[6, 26],
            &[6, 30],
            &[6, 34],
            &[6, 22, 38],
            &[6, 24, 42],
            &[6, 26, 46],
            &[6, 28, 50],
            &[6, 30, 54],
            &[6, 32, 58],
            &[6, 34, 62],
            &[6, 26, 46, 66],
            &[6, 26, 48, 70],
            &[6, 26, 50, 74],
            &[6, 30, 54, 78],
            &[6, 30, 56, 82],
            &[6, 30, 58, 86],
            &[6, 34, 62, 90],
            &[6, 28, 50, 72, 94],
            &[6, 26, 50, 74, 98],
            &[6, 30, 54, 78, 102],
            &[6, 28, 54, 80, 106],
            &[6, 32, 58, 84, 110],
            &[6, 30, 58, 86, 114],
            &[6, 34, 62, 90, 118],
            &[6, 26, 50, 74, 98, 122],
            &[6, 30, 54, 78, 102, 126],
            &[6, 26, 52, 78, 104, 130],
            &[6, 30, 56, 82, 108, 134],
            &[6, 34, 60, 86, 112, 138],
            &[6, 30, 58, 86, 114, 142],
            &[6, 34, 62, 90, 118, 146],
            &[6, 30, 54, 78, 102, 126, 150],
            &[6, 24, 50, 76, 102, 128, 154],
            &[6, 28, 54, 80, 106, 132, 158],
            &[6, 32, 58, 84, 110, 136, 162],
            &[6, 26, 54, 82, 110, 138, 166],
            &[6, 30, 58, 86, 114, 142, 170],
        ];

        return ALIGNMENT_PATTERNS_GRID[*self as usize];
    }
}