use std::str::FromStr;

use sqlx::postgres::PgPoolOptions;
use bigdecimal::BigDecimal;

pub struct Database{
        nome: String,
        valor: BigDecimal,
}

impl Database {
        
        pub fn data()-> Self{
                let nome = "teste".to_string();
                let valor= BigDecimal::from_str("50").unwrap();
                
                Self{nome, valor}             
        
        }

        pub async fn insert(&self)->Result<(), sqlx::Error>{
                let pool = PgPoolOptions::new()
                .connect("postgres://postgres:9263854710@localhost/valuation")
                .await?;

                
                sqlx::query!(
                        "INSERT INTO variaveis (nome, valor) VALUES ($1, $2)",
                        self.nome,
                        self.valor,
                )
                .execute(&pool)
                .await?;

        println!("dado inserido");

        Ok(())
        
        }
    
}