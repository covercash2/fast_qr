mod t_bitstring;
mod t_encoding;
mod t_error_correction;
mod t_generator;
mod t_masks;
mod t_score;
mod t_structure;
mod t_versionformat;

/// Contains all possible generator polynomials (to compule error codewords)
pub const GENERATOR_POLYNOMIALS: [&'static [u8]; 31] = [
    &[0],
    &[0, 0],
    &[0, 25, 1],
    &[0, 198, 199, 3],
    &[0, 75, 249, 78, 6],
    &[0, 113, 164, 166, 119, 10],
    &[0, 166, 0, 134, 5, 176, 15],
    &[0, 87, 229, 146, 149, 238, 102, 21],
    &[0, 175, 238, 208, 249, 215, 252, 196, 28],
    &[0, 95, 246, 137, 231, 235, 149, 11, 123, 36],
    &[0, 251, 67, 46, 61, 118, 70, 64, 94, 32, 45],
    &[0, 220, 192, 91, 194, 172, 177, 209, 116, 227, 10, 55],
    &[0, 102, 43, 98, 121, 187, 113, 198, 143, 131, 87, 157, 66],
    &[
        0, 74, 152, 176, 100, 86, 100, 106, 104, 130, 218, 206, 140, 78,
    ],
    &[
        0, 199, 249, 155, 48, 190, 124, 218, 137, 216, 87, 207, 59, 22, 91,
    ],
    &[
        0, 8, 183, 61, 91, 202, 37, 51, 58, 58, 237, 140, 124, 5, 99, 105,
    ],
    &[
        0, 120, 104, 107, 109, 102, 161, 76, 3, 91, 191, 147, 169, 182, 194, 225, 120,
    ],
    &[
        0, 43, 139, 206, 78, 43, 239, 123, 206, 214, 147, 24, 99, 150, 39, 243, 163, 136,
    ],
    &[
        0, 215, 234, 158, 94, 184, 97, 118, 170, 79, 187, 152, 148, 252, 179, 5, 98, 96, 153,
    ],
    &[
        0, 67, 3, 105, 153, 52, 90, 83, 17, 150, 159, 44, 128, 153, 133, 252, 222, 138, 220, 171,
    ],
    &[
        0, 17, 60, 79, 50, 61, 163, 26, 187, 202, 180, 221, 225, 83, 239, 156, 164, 212, 212, 188,
        190,
    ],
    &[
        0, 240, 233, 104, 247, 181, 140, 67, 98, 85, 200, 210, 115, 148, 137, 230, 36, 122, 254,
        148, 175, 210,
    ],
    &[
        0, 210, 171, 247, 242, 93, 230, 14, 109, 221, 53, 200, 74, 8, 172, 98, 80, 219, 134, 160,
        105, 165, 231,
    ],
    &[
        0, 171, 102, 146, 91, 49, 103, 65, 17, 193, 150, 14, 25, 183, 248, 94, 164, 224, 192, 1,
        78, 56, 147, 253,
    ],
    &[
        0, 229, 121, 135, 48, 211, 117, 251, 126, 159, 180, 169, 152, 192, 226, 228, 218, 111, 0,
        117, 232, 87, 96, 227, 21,
    ],
    &[
        0, 231, 181, 156, 39, 170, 26, 12, 59, 15, 148, 201, 54, 66, 237, 208, 99, 167, 144, 182,
        95, 243, 129, 178, 252, 45,
    ],
    &[
        0, 173, 125, 158, 2, 103, 182, 118, 17, 145, 201, 111, 28, 165, 53, 161, 21, 245, 142, 13,
        102, 48, 227, 153, 145, 218, 70,
    ],
    &[
        0, 79, 228, 8, 165, 227, 21, 180, 29, 9, 237, 70, 99, 45, 58, 138, 135, 73, 126, 172, 94,
        216, 193, 157, 26, 17, 149, 96,
    ],
    &[
        0, 168, 223, 200, 104, 224, 234, 108, 180, 110, 190, 195, 147, 205, 27, 232, 201, 21, 43,
        245, 87, 42, 195, 212, 119, 242, 37, 9, 123,
    ],
    &[
        0, 156, 45, 183, 29, 151, 219, 54, 96, 249, 24, 136, 5, 241, 175, 189, 28, 75, 234, 150,
        148, 23, 9, 202, 162, 68, 250, 140, 24, 151,
    ],
    &[
        0, 41, 173, 145, 152, 216, 31, 179, 182, 50, 48, 110, 86, 239, 96, 222, 125, 42, 173, 226,
        193, 224, 130, 156, 37, 251, 216, 238, 40, 192, 180,
    ],
];
