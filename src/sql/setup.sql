IF (NOT EXISTS (SELECT *
FROM INFORMATION_SCHEMA.TABLES
WHERE TABLE_SCHEMA = 'dbo'
    AND TABLE_NAME = 'Machine'))
BEGIN

    CREATE TABLE Machine
    (
        name nvarchar(50) NOT NULL PRIMARY KEY,
        description nvarchar(100) NOT NULL,
    );
    CREATE TABLE Component
    (
        name nvarchar(50) PRIMARY KEY,
        description nvarchar(100),
        machineName nvarchar(50) NOT NULL,
        priority int NOT NULL DEFAULT 3,
        status int NOT NULL DEFAULT 1,
        FOREIGN KEY (machineName) REFERENCES Machine(name) ON DELETE CASCADE
    );
    CREATE TABLE Log
    (
        id int NOT NULL PRIMARY KEY DEFAULT NEXT VALUE FOR LogIdSeq,
        name nvarchar(50) NOT NULL DEFAULT 'error n'+CONVERT(nvarchar(50),id),
        description nvarchar(100),
        component NVARCHAR(50) NOT NULL,
        startDate datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
        fixedDate datetime,
        status int NOT NULL DEFAULT 1,
        fixed bit NOT NULL DEFAULT 0,
        FOREIGN KEY (component) REFERENCES Component(name) ON DELETE CASCADE
    );
    Create SEQUENCE LogIdSeq AS int START WITH 1 INCREMENT BY 1;
END