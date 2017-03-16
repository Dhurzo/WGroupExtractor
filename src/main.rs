extern crate rusqlite;
extern crate time;

use rusqlite::Connection;
use std::io::prelude::*;
use std::fs::File;

struct Result{
    jid : String,
    subject : String
}

struct PersonResult{
    remote : String
}

fn open_connection() -> rusqlite::Connection
{
	let path ="msgstore.db";
    let conn = Connection::open(path).unwrap();
    return conn;
}

fn main() {

    let mut f = File::create("result.txt").unwrap();
    let conn = open_connection();
    
    let mut results = conn.prepare("SELECT key_remote_jid,subject FROM chat_list").unwrap();
    let result_iter = results.query_map(&[], |row| {
        Result{
            jid: row.get(0),
            subject: row.get(1)
        }
    }).unwrap();

    for result in result_iter{
        let cadena = result.unwrap();
        f.write_fmt(format_args!("Grupo {}: \n " , cadena.subject.split("@").collect::<Vec<&str>>()[0]));
        let mut person_result = conn.prepare("SELECT jid FROM  group_participants WHERE gjid = ? AND gjid IS NOT NULL").unwrap();
        let  mut res = person_result.query_map(&[&cadena.jid], |row| 
        {
            PersonResult{
                remote : row.get(0)
            }
        }).unwrap();
        
        for r in res 
        {
            f.write_fmt(format_args!("\t {} \n",r.unwrap().remote.split("@").collect::<Vec<&str>>()[0]));         
        }
    }
}

