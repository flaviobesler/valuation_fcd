use bigdecimal::BigDecimal;
use std::str::FromStr;
use sqlx::postgres::PgPoolOptions;


pub struct Lancamentos{
        empresa: Option<String>,
        data: Option<chrono::NaiveDate>,
        valor: Option<BigDecimal>,
        ticket: Option<String>,
        variacao_mes_anterior: Option<BigDecimal>,


}

impl Lancamentos{

        pub fn new() ->Self{
                
                let dados = Lancamentos{
                        empresa: None,
                        data: None,
                        valor: None,
                        ticket: None,
                        variacao_mes_anterior: None,
                        

                };
                dados

        }

        pub async fn buscar_valor_anterior(
                        pool: &sqlx::PgPool,
                        ticket: String,
                        data: chrono::NaiveDate,    
                )-> Result<Option<BigDecimal>, sqlx::Error>{
                let resultado = sqlx::query_scalar!(
                        r#"
                        SELECT valor
                        FROM dados_historicos
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


        pub async fn insert(self, resultado: Option<BigDecimal>)->Result<(), sqlx::Error>{
                let pool = PgPoolOptions::new()
                .connect("postgres://postgres:  @localhost/valuation")
                .await?;

                let variacao = match (&self.valor, &resultado) {
                (Some(mes_atual), Some(mes_anterior)) => {
                        Some(
                        ((mes_atual - mes_anterior) / mes_anterior)
                                * BigDecimal::from(100)
                        )
                }
                _ => None,
                };


                sqlx::query!(
                        r#"INSERT INTO dados_historicos 
                                (acao, ultimo_pregao, valor, ticket, variacao_mes)
                        VALUES
                                ($1, $2, $3, $4, $5)
                        "#,
                        self.empresa,
                        self.data,
                        self.valor,
                        self.ticket,
                        variacao,
                )
                .execute(&pool)
                .await?;
                Ok(())
        }



}