-- Your SQL goes here
CREATE TABLE `StatKinds` (
`statKindID` INTEGER PRIMARY KEY,
`sportID` INTEGER ,
`statName` TEXT NOT NULL,
`unit` TEXT NOT NULL,
FOREIGN KEY(`sportID`) REFERENCES Sports(`sportID`) ON DELETE CASCADE 
);


