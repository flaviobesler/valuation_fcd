use bigdecimal::BigDecimal;
use std::str::FromStr;
use sqlx::postgres::PgPoolOptions;

pub struct Indices{
        nome: Option<String>,
        data: Option<chrono::naiveDate>,
        valor: Option<BigDecimal>,
        ticket: Option<String>,
        variacao_mes: Option<BigDecimal>,
}

impl Indices{
        pub fn new()->Self{
                let dados = Indices{
                nome: None,
                data: None,
                valor: None,
                ticket: None,
                variacao_mes: None,
                };
                dados
        }

        pub async fn buscar_valor(
                pool: &sqlx::PgPool,
                ticket: String,
                data: chrono::naiveDate,
        )-> Result<Option<BigDecimal>, sqlx::Error>{
        let resultado = sqlx::query_scalar!(
                r#"
                        SELECT valor
                        FROM historico_indice
                        WHERE ticket = $1
                                AND ultimo_pregao <$2
                        ORDER BY ultimo_pregao DESC
                        LIMIT 1
                "#,
                ticket,
                data
        )
        .fetch_optional(pool)
        .await?;
        Ok(resultado)        
        }

        pub async fn insert(self, resultado: Option<BigDecimal>)-> Result<(), sqlx::Error>{
                let pool = PgPoolOptions::new()
                .connect("postgres://postgres:9263854710@localhost/valuation")
                .await?;
                
                let variacao = match (&self.valor, &resultado){
                        (Some(mes_atual), Some(mes_anterior)) =>{
                                Some(
                                        ((mes_atual - mes_anterior)/ mes_anterior)
                                                * BigDecimal::from(100)
                                )
                               
                        } _ => None,
                
                };

                sqlx::query!(
                        r#"
                                INSERT INTO historico_indice
                                        (nome, valor, ultimo_pregao, variacao_mes, ticket)
                                VALUES
                                        ($1, $2, $3, $4, $5)
                        
                        "#,
                        self.nome, 
                        self.valor,
                        self.data,
                        variacao,
                        self.ticket
                )
                .execute(&pool)
                .await?;
                Ok(())



        }


}