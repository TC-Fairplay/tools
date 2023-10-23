//use str::split;
//use u16::parse;

static DIFF_FEMALE: u16 = 400; // 101 is 'equivalent' to 501.

// see http://www.ahvnummer.ch/ahv-kalender.htm
static MONTH_STARTS: [u16; 12] = [
    101, 132, 163,
    201, 232, 263,
    301, 332, 363,
    401, 432, 463
];

fn string_to_ahv_components(s: &str) -> Option<[u16; 4]> {
    let comps: Vec<u16> = s.split('.').map(parse).collect();
}