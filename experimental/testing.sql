BEGIN TRANSACTION;
DROP TABLE IF EXISTS "companies";
CREATE TABLE IF NOT EXISTS "companies" (
	"id"	INTEGER PRIMARY KEY AUTOINCREMENT,
	"name"	TEXT NOT NULL UNIQUE,
	"address"	TEXT,
	"website"	TEXT,
	"phone"	TEXT,
	"created_date"	TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"last_updated"	TEXT,
	"hide"	INTEGER DEFAULT 0
);
DROP TABLE IF EXISTS "job_postings";
CREATE TABLE IF NOT EXISTS "job_postings" (
	"id"	INTEGER PRIMARY KEY AUTOINCREMENT,
	"link"	TEXT NOT NULL UNIQUE,
	"created_date"	TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"last_updated"	TEXT,
	"description"	TEXT,
	"hide"	INTEGER DEFAULT 0
);
DROP TABLE IF EXISTS "contact_types";
CREATE TABLE IF NOT EXISTS "contact_types" (
	"id"	INTEGER PRIMARY KEY AUTOINCREMENT,
	"name"	TEXT NOT NULL UNIQUE,
	"last_updated"	TEXT,
	"hide"	INTEGER DEFAULT 0
);
DROP TABLE IF EXISTS "contacts";
CREATE TABLE IF NOT EXISTS "contacts" (
	"id"	INTEGER PRIMARY KEY AUTOINCREMENT,
	"name"	TEXT NOT NULL,
	"created_date"	TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"last_updated"	TEXT,
	"email"	TEXT,
	"phone"	TEXT,
	"description"	TEXT,
	"type_id"	INTEGER NOT NULL,
	"hide"	INTEGER DEFAULT 0,
	FOREIGN KEY("type_id") REFERENCES "contact_types"
);
DROP TABLE IF EXISTS "applied_to";
CREATE TABLE IF NOT EXISTS "applied_to" (
	"id"	INTEGER PRIMARY KEY AUTOINCREMENT,
	"date_applied"	TEXT DEFAULT CURRENT_DATE,
	"last_updated"	TEXT,
	"company_id"	INTEGER,
	"job_posting_id"	INTEGER,
	"contact_id"	INTEGER,
	"hide"	INTEGER DEFAULT 0
);
DROP TABLE IF EXISTS "interview_types";
CREATE TABLE IF NOT EXISTS "interview_types" (
	"id"	INTEGER PRIMARY KEY AUTOINCREMENT,
	"name"	TEXT NOT NULL UNIQUE,
	"last_updated"	TEXT,
	"hide"	INTEGER DEFAULT 0
);
DROP TABLE IF EXISTS "interviews";
CREATE TABLE IF NOT EXISTS "interviews" (
	"id"	INTEGER PRIMARY KEY AUTOINCREMENT,
	"interview_type_id"	INTEGER NOT NULL,
	"created_date"	TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"last_updated"	TEXT,
	"date"	TEXT,
	"company_id"	INTEGER NOT NULL,
	"contact_id"	INTEGER,
	"job_posting_id"	INTEGER,
	"description"	TEXT,
	"hide"	INTEGER DEFAULT 0,
	FOREIGN KEY("company_id") REFERENCES "companies",
	FOREIGN KEY("job_posting_id") REFERENCES "job_postings",
	FOREIGN KEY("contact_id") REFERENCES "contacts"
);

DROP TRIGGER IF EXISTS "companies_last_update";
CREATE TRIGGER "companies_last_update" AFTER UPDATE ON companies
BEGIN
	UPDATE companies SET last_updated=CURRENT_TIMESTAMP
	WHERE id = NEW.id;
END;
DROP TRIGGER IF EXISTS "contacts_last_update";
CREATE TRIGGER "contacts_last_update" AFTER UPDATE ON contacts
BEGIN
	UPDATE contacts SET last_updated=CURRENT_TIMESTAMP
	WHERE id = NEW.id;
END;
DROP TRIGGER IF EXISTS "job_postings_last_update";
CREATE TRIGGER "job_postings_last_update" AFTER UPDATE ON job_postings
BEGIN
	UPDATE job_postings SET last_updated=CURRENT_TIMESTAMP
	WHERE id = NEW.id;
END;
DROP TRIGGER IF EXISTS "contact_types_last_update";
CREATE TRIGGER "contact_types_last_update" AFTER UPDATE ON contact_types
BEGIN
	UPDATE contact_types SET last_updated=CURRENT_TIMESTAMP
	WHERE id = NEW.id;
END;
DROP TRIGGER IF EXISTS "applied_to_last_update";
CREATE TRIGGER "applied_to_last_update" AFTER UPDATE ON applied_to
BEGIN
	UPDATE applied_to SET last_updated=CURRENT_TIMESTAMP
	WHERE id = NEW.id;
END;
DROP TRIGGER IF EXISTS "interview_types_last_update";
CREATE TRIGGER "interview_types_last_update" AFTER UPDATE ON interview_types
BEGIN
	UPDATE interview_types SET last_updated=CURRENT_TIMESTAMP
	WHERE id = NEW.id;
END;
DROP TRIGGER IF EXISTS "interviews_last_update";
CREATE TRIGGER "interviews_last_update" AFTER UPDATE ON interviews
BEGIN
	UPDATE interviews SET last_updated=CURRENT_TIMESTAMP
	WHERE id = NEW.id;
END;
COMMIT;
