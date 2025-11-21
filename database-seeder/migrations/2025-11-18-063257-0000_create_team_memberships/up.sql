-- Your SQL goes here

CREATE TABLE `TeamMemberships` (
`membershipID` INTEGER PRIMARY KEY,
`playerID` INTEGER,
`teamID` INTEGER, 
`season` TEXT NOT NULL, 
`jerseyNumber` INTEGER NOT NULL,
FOREIGN KEY(`playerID`) REFERENCES Players(`playerID`) ON DELETE CASCADE ,
FOREIGN KEY(`teamID`) REFERENCES Teams(`teamID`) ON DELETE CASCADE
);
