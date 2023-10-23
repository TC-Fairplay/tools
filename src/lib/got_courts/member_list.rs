use std::{fmt, path::Path};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Entry {
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub phone: String,
    all_data: csv::StringRecord,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.last_name, self.first_name)
    }
}

impl Entry {
    fn get_and_trim(r: &csv::StringRecord, idx: usize) -> String {
        r.get(idx).unwrap().trim().to_string()
    }

    // "Benutzername","Vorname","Nachname","label.email","Geburtsjahr","Telefon (mobil)","Telefon (privat)","Strasse","Strasse 2","Stadt","Postleitzahl","Land","Kategorien","iban","bic","note"
    // "gc_user_123456","Hans","Mustmann",hans.muster@domain.ch,1980,"=""41791234567""",,Musterstrasse 123,,Zürich,8057,CH,label.Mitglieder/Aktiv Einzelmitglied (A),,,
    fn from_got_courts_csv_export(r: csv::StringRecord) -> Entry {
        Entry {
            last_name: Entry::get_and_trim(&r, 2),
            first_name: Entry::get_and_trim(&r, 1),
            email: Entry::get_and_trim(&r, 3),
            phone: Entry::get_and_trim(&r, 5),
            all_data: r,
        }
    }
}

pub fn read_from_csv_export(path: impl AsRef<Path>) -> Vec<Entry> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(path)
        .unwrap();

    reader.records().map(|r| Entry::from_got_courts_csv_export(r.unwrap())).collect()
}
