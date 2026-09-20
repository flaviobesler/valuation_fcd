pub struct Fcff{

        fluxo_caixa_descontado: Option <BigDecimal>,
        fluxo_caixa_livre : Option <BigDecimal>,

}
pub struct valores_banco{
        ebit: Option<BigDecimal>,
        ebt: Option<BigDecimal>,
        tributo: Option<BigDecimal>,
        depreciacao: Option<BigDecimal>,
        capital_giro: Option<BigDecimal>,
        capex: Option<BigDecimal>,
        fco: Option<BigDecimal>,

}

impl Fcff{
        pub fn new() -> Self{
                Self{
                        ebit_12_meses: None,
                        ebt_12_meses: None,
                        taxa: None,
                        depreciacao: None,
                        variacao_capital_giro_12_meses: None,
                        capex: None,

                        fluxo_caixa_descontado: None,
                        fluxo_caixa_livre: None,
                        
                };

        pub async fn dados(&self, pool: &sqlx::PgPool, ticket: String)-> Result<ValoresBanco, sqlx::Error>{
                let ebit = 
                        sqlx::query_scalar!(
                                r#"
                                        SELECT COALESCE(SUM(ebit_trimestre), 0)
                                        FROM (
                                                SELECT ebit_trimestre
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                ORDER BY ano DESC, trimestre DESC
                                                LIMIT 4
                                        )
                                "#,)
                                .fetch_optional(&pool)
                                .await?;
                
                let ebt = 
                        sqlx::query_scalar!(
                                r#"
                                        SELECT COALESCE(SUM(ebt_trimestre),0)
                                        FROM (
                                                SELECT ebt_trimestre
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                ORDER BY ano DESC, trimestre DESC
                                                LIMIT 4 )                               
                                "#,)
                                .fetch_optional(&pool)
                                .await?;
                let tributo =
                        sqlx::query_scalar!(
                                r#"
                                        SELECT COALESCE(SUM(tributo_trimestre),0)
                                        FROM (
                                                SELECT tributo_trimestre
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                ORDER BY ano DESC, trimestre DESC
                                                LIMIT 4 )                               
                                "#,)
                                .fetch_optional(&pool)
                                .await?;
                let depreciacao = 
                        sqlx::query_scalar!(
                                r#"
                                        SELECT COALESCE(SUM(depreciacao_trimestre),0)
                                        FROM (
                                                SELECT depreciacao_trimestre
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                ORDER BY ano DESC, trimestre DESC
                                                LIMIT 4  )       
                                
                                "#,
                        )
                        .fetch_optional(&pool)
                        .await?;
                let capital_giro = 
                        sqlx::query_scalar!(
                                r#"
                                        SELECT COALESCE(SUM(variacao_capital_giro),0)
                                        FROM (
                                                SELECT variacao_capital_giro
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                ORDER BY ano DESC, trimestre DESC
                                                LIMIT 4      )   
                                
                                "#,
                        )
                        .fetch_optional(&pool)
                        .await?;
                let capex = 
                        sqlx::query_scalar!(
                                r#"
                                        SELECT COALESCE(SUM(imob_intang_trimestre),0)
                                        FROM (
                                                SELECT imob_intang_trimestre
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                ORDER BY ano DESC, trimestre DESC
                                                LIMIT 4 )
                                "#,
                        )
                        .fetch_optional(&pool)
                        .await?;

                let fco = 
                        sqlx::query_scalar!(
                                r#"
                                        SELECT COALESCE(SUM(fco_trimestre),0)
                                        FROM (
                                                SELECT fco_trimestre
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                ORDER BY ano DESC, trimestre DESC
                                                LIMIT 4 )
                                "#,
                        )
                        .fetch_optional(&pool)
                        .await?;

                Ok(valores_banco(ebit, ebt, tributo, depreciacao, capital_giro, capex, fco))                      
        
        }

        pub fn calculos(valores_banco: Option<BigDecimal>){
                let taxa = 
                        match(&ebt, &tributo){
                                (Some(ebt), Some(tributo))=>{
                                        Some(tributo / ebt)
                                } _ => None,
                        };
                let nopat = 
                        match(&taxa, &ebit){
                                (Some(taxa), Some(ebit))=>{
                                        Some(ebit * (1- taxa))
                                } _=> None,
                        };
                

                let fcff = 
                        match(&nopat, &depreciacao, &capex, capital_giro){
                                (Some(nopat), Some(depreciacao), Some(capex), Some(capital_giro))=>{
                                        Some(nopat + depreciacao - capex - capital_giro)
                                } _=> None,
                        };

                let fcl = 
                        match (&capex, &fco){
                                (Some(capex), Some(fco))=>{
                                        Some(fco-capex)
                                } _ => None,
                        };
        
        }
        

        


        
        }

}