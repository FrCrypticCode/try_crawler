use crate::request::{Check,req_dreadcast,req_eve};
use mysql::{self, OptsBuilder, prelude::Queryable};
use mysql::{PooledConn,Error};

async fn new()->(Vec<Check>,Vec<String>){
    let mut list:Vec<Check> = vec![];
    let mut logs:Vec<String> = vec![];
    let eve = tokio::spawn(async move {
        let x = req_eve().await;
        return x
    });
    let dc = tokio::spawn(async move {
        let x = req_dreadcast().await;
        return x
    });
    match eve.await{
        Ok(x)=>{
            if let Ok(y) = x{
                list.push(y);
            }
            else if let Err(err) = x{
                logs.push(err.to_string());
            }
        },
        Err(err)=>{logs.push(err.to_string())}
    }
    match dc.await{
        Ok(x)=>{
            if let Ok(y) = x{
                list.push(y);
            }
            else if let Err(err) = x{
                logs.push(err.to_string());
            }
        },
        Err(err)=>{logs.push(err.to_string())}
    }
    return (list,logs);
}

pub async fn init(){
    let (t,mut logs) = new().await;
    let opts = OptsBuilder::new()
        .user(Some("root"))
        .pass(Some("*********"))
        .ip_or_hostname(Some("127.0.0.1"))
        .db_name(Some("crawler"));
    
    let pool = mysql::Pool::new(opts);
    match pool{
        Ok(p)=>{
            let co = p.get_conn();
            match co{
                Ok(mut c)=>{
                    for check in t{
                        let query = String::from("SELECT vers FROM refs WHERE website='")+&check.ws+"'";
                        let res:Result<Vec<String>,Error> = c.query(query);
                        match res{
                            Ok(x)=>{
                                let mut new = true;
                                for p in x{
                                    if p == check.version{
                                        new = false;
                                        break;
                                    }
                                }
                                if new{
                                    // Insérer une nouvelle ligne
                                    send_d(&mut c, check).await;
                                    continue
                                }
                            },
                            Err(err)=>{logs.push(err.to_string())}
                        }
                    }
                },
                Err(err)=>{logs.push(err.to_string())}
            }
        },
        Err(err)=>{logs.push(err.to_string())}
    }
    for l in logs{
        println!("Erreur : {}",l)
    }
    
}

async fn send_d(c:&mut PooledConn,data:Check){
    // SQL Fonctionnel
    // INSERT INTO refs(vers,date_r,summary,website) VALUES ('test','test','test','test');

    match c.prep("INSERT INTO refs(vers, date_r, summary, website) VALUES (?, ?, ?, ?)"){
        Ok(x)=>{
            match c.exec_iter(x,(data.version,data.date,data.resume,data.ws)){
                Ok(_x)=>{
                    println!("Nouvelle entrée ajoutée.")
                },
                Err(err)=>{
                    println!("Erreur : {}",err.to_string());
                }
            }
        },
        Err(err)=>{println!("Erreur : {}",err.to_string());}
    }
    
}