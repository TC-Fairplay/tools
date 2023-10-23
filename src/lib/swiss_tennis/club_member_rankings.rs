use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::io::BufReader;
use std::{path::Path, vec::Vec};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Player {
    last_name: String,
    first_name: String,
    classification: String,
    classification_value: f32,
}

impl<'de> Deserialize<'de> for Player {
    fn deserialize<D>(deserializer: D) -> Result<Player, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct JsonPlayer {
            classification: String,
            classification_value: f32,
            person: Person,
        }

        #[derive(Deserialize)]
        struct Person {
            #[serde(rename = "lastname")]
            last_name: String,
            #[serde(rename = "firstname")]
            first_name: String,
        }

        let p = JsonPlayer::deserialize(deserializer)?;
        Ok(Player {
            last_name: p.person.last_name,
            first_name: p.person.first_name,
            classification: p.classification,
            classification_value: p.classification_value,
        })
    }
}

struct PlayersWrapper(Vec<Player>);

impl<'de> Deserialize<'de> for PlayersWrapper {
    fn deserialize<D>(deserializer: D) -> Result<PlayersWrapper, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct JsonPlayers {
            data: Inner,
        }

        #[derive(Deserialize)]
        struct Inner {
            #[serde(rename = "lizenz_nehmer")]
            players: Vec<Player>,
        }

        let ps = JsonPlayers::deserialize(deserializer)?;
        Ok(PlayersWrapper(ps.data.players))
    }
}

pub fn read_players(path: impl AsRef<Path>) -> Vec<Player> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let players: PlayersWrapper = serde_json::from_reader(reader).unwrap();
    players.0
}
