use log::info;
use super::fairgate::contacts::Member;
use super::got_courts::member_list::Entry;

fn match_fuzzily(source: &Member, copy: &Entry) -> bool {
    fn lower(s: &str) -> String {
        s.to_lowercase()
    }

    let (nx, ny) = (lower(&source.last_name), lower(&copy.last_name));
    let (fnx, fny) = (lower(&source.first_name), lower(&copy.first_name));

    let equals = || nx == ny && fnx == fny;
    let starts_with = || {
        (nx.starts_with(&ny) && fnx.starts_with(&fny))
            || (ny.starts_with(&nx) && fny.starts_with(&fnx))
    };
    let ends_with = || {
        (nx.ends_with(&ny) && fnx.ends_with(&fny))
            || (ny.ends_with(&nx) && fny.ends_with(&fnx))
    };
    let short_edit_dist =
        || edit_distance::edit_distance(&nx, &ny) + edit_distance::edit_distance(&fnx, &fny) < 2;

    if equals() {
        true
    } else if starts_with() {
        info!("Matched '{}' with '{}' (starts_with).", source, copy);
        true
    } else if ends_with() {
        info!("Matched '{}' with '{}' (ends_with).", source, copy);
        true
    } else if short_edit_dist() {
        info!("Matched '{}' with '{}'. (edit_distance < 2)", source, copy);
        true
    } else {
        false
    }
}

pub fn print_matching_items(members: Vec<Member>, entries: Vec<Entry>) {
    println!("## No TC Fairplay entry found for the following GC Accounts:");
    for e in entries {
        match members.iter().find(|tc| match_fuzzily(tc, &e)) {
            Some(_) => (),
            None => println!("{}, {}", e.last_name, e.first_name),
        }
    }
}
