-- Your SQL goes here
CREATE TABLE `StatInstances` (
`statInstanceID` INTEGER PRIMARY KEY,
`playerID` INTEGER,
`gameID` INTEGER, 
`statKindID` INTEGER NOT NULL,
`timestamp` TEXT,
`value` REAL NOT NULL,
FOREIGN KEY(`statKindID`) REFERENCES StatKinds(`statKindID`) ON DELETE CASCADE,
FOREIGN KEY(`playerID`) REFERENCES Players(`playerID`) ON DELETE CASCADE,
FOREIGN KEY(`gameID`) REFERENCES Games(`gameID`) ON DELETE CASCADE
);


