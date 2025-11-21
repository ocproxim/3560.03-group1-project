-- Your SQL goes here
CREATE TABLE `StatInstances` (
`statInstanceID` INTEGER PRIMARY KEY,
`playerID` INTEGER,
`gameID` INTEGER, 
`statKindID` INTEGER NOT NULL,
`timestamp` TEXT,
`value` REAL NOT NULL,
FOREIGN KEY(`playerID`) REFERENCES Players(`playerID`),
FOREIGN KEY(`gameID`) REFERENCES Games(`gameID`)
);


