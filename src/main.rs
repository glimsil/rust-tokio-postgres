use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=test1234", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT name, description FROM test", &[])
        .await?;

    for row in &rows {
        let name: &str = row.get(0);
        let description: &str = row.get(1);
        println!("{}", format!("name='{}' description='{}'", name, description))
    }
    
    Ok(())
}
