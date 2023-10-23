use csv::{QuoteStyle, WriterBuilder};
use std::path::Path;
use itertools::Itertools;
use crate::fairgate::contacts::{Member, MemberType};

pub fn write_to_csv(list: &[Member], p: impl AsRef<Path>, include_all_data: bool) {
    let mut w = WriterBuilder::new()
        .quote_style(QuoteStyle::Always)
        .from_path(p)
        .unwrap();

    for m in list {
        let name = format!("{}, {}", m.last_name, m.first_name);

        if include_all_data {
            w.write_record([
                name,
                m.phone_number.clone().unwrap_or_default(),
            ])
            .unwrap();
        } else {
            w.write_record([name]).unwrap();
        }
    }
    w.flush().unwrap();
}

static LISTS: [(MemberType, &str, bool); 3] = [
    (MemberType::Active, "aktive.csv", true),
    (MemberType::JuniorA, "junioren_a.csv", false),
    (MemberType::JuniorB, "junioren_b.csv", false),
];

pub fn write_website_csv_files(
    members: Vec<Member>,
    base_path: impl AsRef<Path>,
) {
    // group by member type
    let mut map = members.into_iter().into_group_map_by(|m| m.member_type);

    // sort all lists by name
    for vec in map.values_mut() {
        vec.sort_by_key(|m| (m.last_name.clone(), m.first_name.clone()));
    }

    let base_path = base_path.as_ref();
    for (mt, file_name, all_data) in LISTS {
        write_to_csv(
            &map[&mt],
            base_path.join(file_name),
            all_data,
        );
    }
}
