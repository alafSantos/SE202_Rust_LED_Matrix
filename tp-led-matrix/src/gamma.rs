pub const GAMMA_TAB: [u8; 256] = [
    0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x02, 0x02, 0x03,
    0x03, 0x03, 0x03, 0x04, 0x04, 0x04, 0x04, 0x05, 0x05, 0x05, 0x06, 0x06, 0x06, 0x07, 0x07, 0x08,
    0x08, 0x08, 0x09, 0x09, 0x0a, 0x0a, 0x0b, 0x0b, 0x0b, 0x0c, 0x0c, 0x0d, 0x0d, 0x0e, 0x0e, 0x0f,
    0x0f, 0x10, 0x11, 0x11, 0x12, 0x12, 0x13, 0x13, 0x14, 0x15, 0x15, 0x16, 0x16, 0x17, 0x18, 0x18,
    0x19, 0x1a, 0x1a, 0x1b, 0x1c, 0x1c, 0x1d, 0x1e, 0x1e, 0x1f, 0x20, 0x20, 0x21, 0x22, 0x23, 0x23,
    0x24, 0x25, 0x26, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2e, 0x2f, 0x30,
    0x31, 0x32, 0x33, 0x34, 0x35, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f,
    0x40, 0x41, 0x42, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4c, 0x4d, 0x4e, 0x4f,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5c, 0x5d, 0x5e, 0x5f, 0x60,
    0x61, 0x62, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x71, 0x72, 0x73,
    0x74, 0x75, 0x77, 0x78, 0x79, 0x7a, 0x7c, 0x7d, 0x7e, 0x7f, 0x81, 0x82, 0x83, 0x85, 0x86, 0x87,
    0x89, 0x8a, 0x8b, 0x8c, 0x8e, 0x8f, 0x91, 0x92, 0x93, 0x95, 0x96, 0x97, 0x99, 0x9a, 0x9b, 0x9d,
    0x9e, 0xa0, 0xa1, 0xa2, 0xa4, 0xa5, 0xa7, 0xa8, 0xaa, 0xab, 0xac, 0xae, 0xaf, 0xb1, 0xb2, 0xb4,
    0xb5, 0xb7, 0xb8, 0xba, 0xbb, 0xbd, 0xbe, 0xc0, 0xc1, 0xc3, 0xc4, 0xc6, 0xc7, 0xc9, 0xca, 0xcc,
    0xcd, 0xcf, 0xd1, 0xd2, 0xd4, 0xd5, 0xd7, 0xd8, 0xda, 0xdc, 0xdd, 0xdf, 0xe0, 0xe2, 0xe4, 0xe5,
    0xe7, 0xe9, 0xea, 0xec, 0xee, 0xef, 0xf1, 0xf3, 0xf4, 0xf6, 0xf8, 0xf9, 0xfb, 0xfd, 0xfe, 0xff,
];

pub fn gamma_correct(x: u8) -> u8 {
    return GAMMA_TAB[x as usize];
}
