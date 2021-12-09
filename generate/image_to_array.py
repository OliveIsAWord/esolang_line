prelude = """

// @generated
// Includes: {files}
// Auto-generated on {time}
// Not intended for manual editing

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BitRaster<'a> {{
    pub width: usize,
    pub height: usize,
    pub offset_x: usize,
    pub offset_y: usize,
    pub pixels: &'a [bool],
}}

impl BitRaster<'_> {{
    pub fn get(&self, x: usize, y: usize) -> bool {{
        self.pixels[y * self.width + x]
    }}
}}

const W: bool = true;
#[allow(non_upper_case_globals)]
const o: bool = false;

""".strip()

raster_template = """
const {name}_BITS: [bool; {width} * {height}] = [
{bool_bits}
];

pub const {name}: BitRaster = BitRaster {{
    width: {width},
    height: {height},
    offset_x: {offset_x},
    offset_y: {offset_y},
    pixels: &{name}_BITS,
}};
""".strip()

def get_prelude(filenames):
    from datetime import datetime
    now_str = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
    return prelude.format(time=now_str, files=' '.join(filenames))

def to_bool_bits(img, tab_level=1):
    tab = "    " * tab_level
    lines = []
    for y in range(img.height):
        bools = ('W,' if img.getpixel((x, y))[2] == 0 else 'o,'
                 for x in range(img.width))
        lines.append(tab + ''.join(bools))
    return '\n'.join(lines)

def find_offset(img):
    for x in range(img.width):
        for y in range(img.height):
            if img.getpixel((x, y)) == (255, 0, 0):
                return (x, y)
    return None
            

if __name__ == "__main__":
    import sys
    argv = sys.argv
    if len(argv) < 2:
        print(f'USAGE: {argv[0]} FILES...', file=sys.stderr)
        sys.exit(-8)
    from PIL import Image
    codes = []
    for filepath in argv[1:]:
        name = filepath.split('.')[0].upper()
        with Image.open(filepath) as img:
            img = img.convert('RGB')
            width, height = img.width, img.height
            bool_bits = to_bool_bits(img)
            offset_x, offset_y = find_offset(img)
            
        code = raster_template.format(
            name = name,
            bool_bits = bool_bits,
            width = width,
            height = height,
            offset_x = offset_x,
            offset_y = offset_y,
            )
        codes.append(code)
    print(get_prelude(argv[1:]), *codes, sep='\n\n\n')
    
    
