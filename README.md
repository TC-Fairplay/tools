# CLI Tools

🎾 Command line tools for various internal tasks.

## Tasks

### Match Member Lists

Match the list of club members exported (as Excel file) from the [Fairgate](https://mein.fairgate.ch) database to the one exported from the [GotCourts](https://www.gotcourts.com) system. All entries are shown which are only in one of the two lists.

### Export Member Lists

Write all relevant club member data from the Fairgate system (exported as Excel file) to a CSV file to be processed by the [member-listing](https://www.github.com/tc-fairplay/member-listing) tool.

### Send Mails

Send a [markdown](https://www.markdownguide.org/)-formatted email to a subset of all club members. Used for tournament announcements.

🚧 This is still a work in progress.

### SwissTennis Rankings

Import a list of players (provided as JSON file) from the SwissTennis website.

🚧 This is a work in progress.

## External Data Sources

- ['MyTennis' area](https://www.mytennis.ch) of [SwissTennis](https://www.swisstennis.ch) (see directory [src/lib/swiss_tennis](src/lib/swiss_tennis)).
- [Fairgate](https://mein.fairgate.ch/tcfairplay) (filter "Aktuelle Mitglieder", see  directory [src/lib/fairgate](src/lib/fairgate)).
- [GotCourts](https://www.gotcourts.com) (see directory [src/lib/got_courts](src/lib/got_courts))
