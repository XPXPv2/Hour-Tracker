use rusqlite::Connection;

pub fn new_db(p: &str) -> Connection {
    std::fs::File::create(p).unwrap();
    let con = Connection::open(p).unwrap();

    con.execute(
        "CREATE TABLE people(\
                    id INTEGER PRIMARY KEY NOT NULL,\
                    first TEXT collate NOCASE,\
                    mi TEXT collate NOCASE,\
                    last TEXT collate NOCASE,\
                    class INTEGER,\
                    inducted INTEGER\
                 );",
        [],
    )
    .unwrap();

    con.execute(
        "CREATE TABLE inductions(\
                    id INTEGER PRIMARY KEY NOT NULL,\
                    first TEXT collate NOCASE,\
                    last TEXT collate NOCASE,\
                    year INTEGER NOT NULL,\
                    grade INTEGER NOT NULL,\
                    returned BOOLEAN NOT NULL,\
                    inducted BOOLEAN NOT NULL,\
                    notes TEXT collate NOCASE,\
                    ceramony BOOLEAN,\
                    school_id INTEGER,\
                    mi TEXT collate NOCASE\
                 );",
        [],
    )
    .unwrap();

    con.execute(
        "CREATE TABLE service(\
                    id  INTEGER NOT NULL,\
                    year INTEGER NOT NULL,\
                    semester BOOLEAN NOT NULL,\
                    personal BOOLEAN NOT NULL,\
                    hours INTEGER NOT NULL\
                 );",
        [],
    )
    .unwrap();

    con
}
