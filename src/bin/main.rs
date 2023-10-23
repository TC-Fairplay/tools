use lib::{got_courts, fairgate, swiss_tennis, tc_fairplay, member_matcher, smtp};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    SwissTennisRankings {
        json_path: String,
    },
    MatchMemberLists {
        fairgate_excel_path: String,
        got_courts_csv_path: String,
    },
    ExportMemberLists {
        excel_path: String,
        output_path: String,
    },
    SendMail {
        markdown_path: String,

        // SMTP server and credentials
        server: String,
        username: String,
        password: String,
    }
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    match args.command {
        Command::ExportMemberLists {
            excel_path,
            output_path,
        } => {
            let members = fairgate::contacts::read_from_excel_file(excel_path);
            tc_fairplay::club_members_list::write_website_csv_files(members, output_path);
        }

        Command::MatchMemberLists {
            fairgate_excel_path,
            got_courts_csv_path,
        } => {
            let members = fairgate::contacts::read_from_excel_file(fairgate_excel_path);
            let entries = got_courts::member_list::read_from_csv_export(got_courts_csv_path);
            member_matcher::print_matching_items(members, entries);
        }

        Command::SwissTennisRankings { json_path } => {
            let players = swiss_tennis::club_member_rankings::read_players(json_path);
            for p in &players {
                println!("{:#?}", p);
            }

            println!("Number of players: {}", players.len());
        }

        Command::SendMail { markdown_path, server, username, password } => {
            let cfg = smtp::SmtpConfig { server, username, password };
            smtp::send_mail(&cfg, &markdown_path);
        }
    }
}
