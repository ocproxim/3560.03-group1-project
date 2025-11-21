-- Your SQL goes here
CREATE TABLE `Users` (
`userID` INTEGER PRIMARY KEY NOT NULL, 
`email` TEXT NOT NULL,
`passwordHash` TEXT NOT NULL,
`role` INTEGER NOT NULL
);
