extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

use std::collections::HashMap;
use std::panic;

#[derive(Debug)]
struct Spell {
    name: String,
    command: String,
}

// pub struct MyDbContext<'stat>{
//     pub conn: &'stat Connection,
//   }
//
//   impl<'stat> MyDbContext<'stat>
//   {
//     pub fn new(conn: &'stat Connection) -> Self
//     {
//       return MyDbContext {
//         conn,
//       };
//     }
//   }

pub struct MyDbContext{
    pub conn: Connection,
}

impl MyDbContext
{
    pub fn new(conn: Connection) -> Self
    {
        return MyDbContext {
            conn,
        };
    }
}

fn main() -> Result<()> {
    // let conn = Connection::open("spellbook.db")?;
    // let context = MyDbContext::new(&conn);
    let context = connect_db();
    //std::result::Result<rusqlite::Connection, rusqlite::Error>
//     let x: Result<i32, &str> = Err("Some error message");
// assert_eq!(x.is_ok(), false);
//     let conn: std::result::Result<rusqlite::Connection, rusqlite::Error> = connect_db();

    context.conn.execute(
        "create table if not exists tag (
             id integer primary key,
             name text not null
         )",
        NO_PARAMS,
    )?;

    context.conn.execute(
        "create table if not exists spell (
             id integer primary key,
             tag_id integer null references tag(id),
             name text not null,
             command text
         )",
        NO_PARAMS,
    )?;

    insert_data(&context)?;

    Ok(())
}

fn connect_db () -> MyDbContext {
    let conn = Connection::open("spellbook.db");
    let context = MyDbContext::new(conn.unwrap());

    context
}

fn insert_data(dbconn: &MyDbContext) -> Result<()> {
    //let conn = Connection::open("spellbook.db")?;

    let mut spells = HashMap::new();
    spells.insert(String::from("kubectl delete pod"), vec!["kubectl delete pod x --namespace test"]);
    spells.insert(String::from("ssh into server"), vec!["ssh -i ~/ssh/somekey.pub bhenkel@19.12.2.4"]);

    for (spellname, spellcommand) in &spells {
        dbconn.conn.execute(
            "INSERT INTO spell (name, command) values (?1, ?2)",
            &[&spellname, &spellcommand[0].to_string()],
        )?;
    }

    Ok(())
}

// fn select_data () -> Result<()> {
//     let mut stmt = conn.prepare(
//         "SELECT c.name, cc.name from cats c
//          INNER JOIN cat_colors cc
//          ON cc.id = c.color_id;",
//     )?;
    
//     let cats = stmt.query_map(NO_PARAMS, |row| {
//         Ok(Cat {
//             name: row.get(0)?,
//             color: row.get(1)?,
//         })
//     })?;
    
//     for cat in cats {
//         println!("Found cat {:?}", cat);
//     }
    
//     Ok(())
// }


fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::path::Path;

    fn setup() {
        let conn = Connection::open("testing.db");
        println!("setup()");
    }
    fn teardown() {
        use std::fs;
        fs::remove_file("testing.db").unwrap();
        println!("teardown()");
    }

    fn run_test<T>(test: T) -> ()
        where T: FnOnce() -> () + panic::UnwindSafe
    {
        setup();

        let result = panic::catch_unwind(|| {
            test()
        });

        teardown();

        assert!(result.is_ok())
    }

    #[test]
    fn test_add() {
        run_test(|| {
          assert_eq!(add(1, 2), 3);
        });
    }

    #[test]
    fn test_connect_db () {
        run_test(|| {
            let conn = Connection::open("testing.db");
            let context = MyDbContext::new(conn.unwrap());
            assert_eq!(true, Path::new("testing.db").exists());

            context.conn.execute(
                "create table if not exists tag (
             id integer primary key,
             name text not null
         )",
                NO_PARAMS,
            ).unwrap();  //TODO look into the ramifications of unwrapping. My current understanding it will panic anc crash the app if an error is found.
        });

    }

}
