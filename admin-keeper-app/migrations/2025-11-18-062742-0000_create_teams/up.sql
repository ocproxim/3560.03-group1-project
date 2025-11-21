-- Your SQL goes here
CREATE TABLE `Teams` (
`teamID` INTEGER PRIMARY KEY,
`sportID` INTEGER,
`teamName` TEXT NOT NULL, 
`homeTown` TEXT NOT NULL,
FOREIGN KEY(`sportID`) REFERENCES Sports(`sportID`)
);
