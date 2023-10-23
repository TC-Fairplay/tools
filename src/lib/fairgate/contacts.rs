use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use lazy_static::lazy_static;
use regex::{Match, Regex};
use serde::Deserialize;
use std::{path::Path, str::FromStr, fmt::{Display, Formatter, self}};

#[derive(Debug)]
pub struct Member {
    pub member_type: MemberType,
    pub last_name: String,
    pub first_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MemberType {
    Active,
    Passive,
    JuniorA,
    JuniorB,
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.last_name, self.first_name)
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct RawExcelRow {
    #[serde(rename = "Kontakte")]
    contact: String,

    #[serde(rename = "Mitgliedschaft")]
    member_type_code: String,

    #[serde(rename = "Nachname")]
    last_name: String,
    #[serde(rename = "Vorname")]
    first_name: String,

    #[serde(rename = "Primäre E-Mail")]
    email: Option<String>,

    #[serde(rename = "Handy")]
    phone_number: Option<String>,
}

impl From<RawExcelRow> for Member {
    fn from(row: RawExcelRow) -> Member {
        // if there is a phone number, it must have the right format
        let phone_number = row.phone_number.map(|s| {
            PhoneNumber::from_str(&s)
                .unwrap_or_else(|_| panic!("Phone format must be valid. Found: '{}'", &s))
                .to_string()
        });

        Member {
            member_type: MemberType::from(row.member_type_code.as_ref()),
            last_name: row.last_name,
            first_name: row.first_name,
            email: row.email.map(|s| s.to_lowercase()),
            phone_number
        }
    }
}

impl From<&str> for MemberType {
    fn from(s: &str) -> MemberType {
        match s {
            "ae" | "ak" | "bf" | "se" | "ts" | "ur" | "st2" => MemberType::Active,
            "pz" => MemberType::Passive,
            "ja" => MemberType::JuniorA,
            "jb" => MemberType::JuniorB,
            _ => panic!("Unknown member code: {}", s),
        }
    }
}

struct PhoneNumber(String);

struct PhoneNumberParseError;

impl FromStr for PhoneNumber {
    type Err = PhoneNumberParseError;

    fn from_str(s: &str) -> Result<PhoneNumber, PhoneNumberParseError> {
        // Input examples:
        // - +41 79 123 45 67, +49761234567
        // - 076/123 45 67, 0761234556
        lazy_static! {
            static ref PHONE_REGEX: Regex =
                Regex::new(r"^((?:\+|(?:00))\d{2})?\s*(\d{2,3})[/\s]*(\d{3})\s*(\d{2})\s*(\d{2})$")
                    .unwrap();
        }
        let s = s.trim();
        PHONE_REGEX.captures(s).map(|caps| {
            let formatted = caps.iter()
                .skip(1)
                .filter_map(|o| o.map(|m: Match| m.as_str()))
                //.filter_map(|o| o.map(Match::as_str))
                .fold(String::new(), |a, b| a + " " + b)
                .trim_start()
                .to_string();

            PhoneNumber(formatted)
        }).ok_or(PhoneNumberParseError)
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn read_from_excel_file(path: impl AsRef<Path>) -> Vec<Member> {
    // open excel file
    let mut excel: Xlsx<_> = open_workbook(path).unwrap();

    // get first sheet
    let range = excel.worksheet_range_at(0).unwrap().unwrap();

    // read all rows, i.e. members
    let members: Vec<Member> = RangeDeserializerBuilder::new()
        .has_headers(true)
        .from_range::<_, RawExcelRow>(&range)
        .unwrap()
        .map(|item| Member::from(item.unwrap()))
        .collect();

    members
}
