use bigdecimal::BigDecimal;
use std::str::FromStr;
use sqlx::postgres::PgPoolOptions;

pub struct Capex_historico{
        ticket: Option<String>,
        ano: Option<BigDecimal>,
        trimestre: Option<BigDecimal>,
        
        //vai pro app direto
        fco_acumulado: Option<BigDecimal>,
        depreciacao_acumulada: Option<BigDecimal>,
        imob_intang_acumulado: Option<BigDecimal>,
        ebt_acumulado: Option<BigDecimal>,
        ebit_acumulado: Option<BigDecimal>,
        tributo_acumulado: Option <BigDecimal>,


        //calculado para o banco
        tributo_trimestre: Option <BigDecimal>,
        ativo_circulante_trimestre: Option<BigDecimal>,
        passivo_circulante_trimestre: Option<BigDecimal>,
        capital_giro_liquido_trimestre: Option<BigDecimal>,
        variacao_capital_giro: Option<BigDecimal>,

        ebit_trimestre : Option<BigDecimal>,
        fco_trimestre: Option<BigDecimal>,
        depreciacao_trimestre: Option<BigDecimal>,
        imob_intang_trimestre: Option<BigDecimal>,
        ebt_trimestre: Option<BigDecimal>,
}

pub struct Valores_historicos{
        pub fco: Option<BigDecimal>,
        pub depreciacao: Option<BigDecimal>,
        pub imob_intang: Option<BigDecimal>,
        pub ebt: Option<BigDecimal>,
        pub capital_giro_liquido: Option<BigDecimal>,
        pub ebit: Option<BigDecimal>,
        pub tributo: Option <BigDecimal>,

}

impl Capex_historico{
        pub fn new()-> Self{
                Self{
                        ano: None,
                        trimestre: None,
                        
                        //vai pro app direto
                        fco_acumulado:  None,
                        depreciacao_acumulada:  None,
                        imob_intang_acumulado:  None,
                        ebt_acumulado: None,
                        ebit_acumulado:  None,
                        tributo_acumulado:  None,


                        //calculado para o banco
                        tributo_trimestre:  None,
                        ativo_circulante_trimestre:  None,
                        passivo_circulante_trimestre:  None,
                        capital_giro_liquido_trimestre:  None,
                        variacao_capital_giro: None,

                        ebit_trimestre :  None,
                        fco_trimestre:  None,
                        depreciacao_trimestre:  None,
                        imob_intang_trimestre:  None,
                        ebt_trimestre:  None,
                
                }
        }
        pub async fn buscar_valores(
                        &self,
                        pool:&sqlx::PgPool
                )-> Result<Valores_historicos, sqlx::Error>{
                let fco = 
                        if self.trimestre() > BigDecimal::from(1){
                        sqlx::query_scalar!(
                                r#"
                                        SELECT fco_acumulado 
                                        FROM ffcf_historico
                                        WHERE ticket = $1
                                                AND ano = $2
                                                AND trimestre <$3
                                        ORDER BY trimestre DESC
                                        LIMIT 1
                                "#,
                                self.ticket,
                                self.ano, 
                                self.trimestre
                        
                        )
                        .fetch_optional(pool)
                        .await?};
                        
                
                let depreciacao = 
                        if self.trimestre() > BigDecimal::from(1){
                                sqlx::query_scalar!(
                                        r#"
                                                SELECT depreciacao_acumulada
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                        AND ano = $2
                                                        AND trimestre <$3
                                                ORDER BY trimestre DESC
                                                LIMIT 1
                                        "#,
                                        self.ticket,
                                        self.ano,
                                        self.trimestre
                                )
                                .fetch_optional(pool)
                                .await?;
                        };
                
                let imob_intang = 
                        if self.trimestre() >BigDecimal::from(1){
                                sqlx::query_scalar!(
                                        r#"
                                                SELECT  imob_intang_acumulado
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                        AND ano = $2
                                                        AND trimestre <$3
                                                ORDER BY trimestre DESC
                                                LIMIT 1
                                        "#,
                                        self.ticket,
                                        self.ano,
                                        self.trimestre
                                )
                                .fetch_optional(pool)
                                .await?;
                        };

                let ebit = 
                        if self.trimestre() > BigDecimal::from(1){
                                sqlx::query_scalar!(
                                        r#"
                                                SELECT  ebit_acumulado
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                        AND ano = $2
                                                        AND trimestre <$3
                                                ORDER BY trimestre DESC
                                                LIMIT 1
                                        "#,
                                        self.ticket,
                                        self.ano,
                                        self.trimestre
                                )
                                .fetch_optional(pool)
                                .await?;
                        
                        };

                let ebt = 
                        if self.trimestre() >BigDecimal::from(1){
                                sqlx::query_scalar!(
                                        r#"
                                                SELECT  ebt_acumulado
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                        AND ano = $2
                                                        AND trimestre <$3
                                                ORDER BY trimestre DESC
                                                LIMIT 1
                                        "#,
                                        self.ticket,
                                        self.ano,
                                        self.trimestre
                                )
                                .fetch_optional(pool)
                                .await?;
                        };

                let capital_giro_liquido = 
                        if self.trimestre() >BigDecimal::from(1){
                                sqlx::query_scalar!(
                                        r#"
                                                SELECT capital_giro_liquido_trimestre
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                        AND ano = $2
                                                        AND trimestre <$3
                                                ORDER BY trimestre DESC
                                                LIMIT 1
                                        "#,
                                        self.ticket,
                                        self.ano,
                                        self.trimestre
                                )
                                .fetch_optional(pool)
                                .await?
                        }else{
                                sqlx::query_scalar!(
                                        r#"
                                                SELECT capital_giro_liquido_trimestre
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                        AND ano = $2 -1
                                                        AND trimestre =4
                                                ORDER BY trimestre DESC
                                                LIMIT 1
                                        "#,
                                        self.ticket,
                                        self.ano,
                                )
                                .fetch_optional(pool)
                                .await?
                        };
                        
                        let tributo = 
                                if self.trimestre() > BigDecimal::from(1){
                                        sqlx::query_scalar!(
                                          r#"
                                                SELECT tributo_trimestre
                                                FROM ffcf_historico
                                                WHERE ticket = $1
                                                        AND ano = $2
                                                        AND trimestre <$3
                                                ORDER BY trimestre DESC
                                                LIMIT 1
                                        "#,
                                        self.ticket,
                                        self.ano,
                                        self.trimestre
                                                                     
                                        
                                        )
                                
                                };



                        Ok(Valores_historicos(
                                fco, depreciacao, imob_intang, ebt, capital_giro_liquido, tributo, ebit
                        ))
        }

        pub fn calculos(
                &mut self,
                Valores_historicos: Option<BigDecimal>,
        ) {
                self.ebit_trimestre = 
                        if self.trimestre ()> BigDecimal::from(1){
                                match(&self.ebit_acumulado, &ebit){
                                        (Some(ebit_atual), Some(ebit_banco))=>{
                                                Some(ebit_atual - ebit_banco)
                                        }
                                        _ => None,
                                }
                        } else{
                                self.ebit_acumulado.clone()
                        };

                
                self.tributo_trimestre = 
                        if self.trimestre() > BigDecimal::from(1){
                                match(&self.tributo_acumulado, &tributo){
                                        (Some(tributo_atual), Some(tributo_banco))=>{
                                                Some(tributo_atual - tributo_banco)
                                        }
                                        _ => None,
                                }
                        }else{
                                self.tributo_acumulado.clone()
                        };

                self.fco_trimestre =
                        if self.trimestre() > BigDecimal::from(1) {
                        match (&self.fco_acumulado, &fco) {
                                (Some(fco_atual), Some(fco_banco)) => {
                                Some(fco_atual - fco_banco)
                                }
                                _ => None,
                        }
                        } else {
                        self.fco_acumulado.clone()
                        };

                self.depreciacao_trimestre =
                        if self.trimestre() > BigDecimal::from(1) {
                        match (&self.depreciacao_acumulada, &depreciacao) {
                                (Some(depreciacao_atual), Some(depreciacao_banco)) => {
                                Some(depreciacao_atual - depreciacao_banco)
                                }
                                _ => None,
                        }
                        } else {
                        self.depreciacao_acumulada.clone()
                        };

                self.imob_intang_trimestre = 
                        if self.trimestre() > BigDecimal::from(1){
                                match(&self.imob_intang_acumulado, &imob_intang){
                                        (Some(imob_atual), Some(imob_banco))=>{
                                                Some(imob_atual - imob_banco)
                                        }
                                        _ => None,
                                }
                        }else{
                                self.imob_intang_acumulado.clone()
                        };

                self.ebt_trimestre = 
                        if self.trimestre()> BigDecimal::from(1){
                                match(&self.ebt_acumulado, &ebt){
                                        (Some(ebt_atual), Some(ebt_banco))=>{
                                                Some(ebt_atual - ebt_banco)
                                        }
                                        _ => None,
                                }
                        }else{
                                self.ebt_acumulado.clone()
                        };

                self.capital_giro_liquido_trimestre =
                        match (
                                &self.ativo_circulante_trimestre,
                                &self.passivo_circulante_trimestre
                        ) {
                                (Some(ac), Some(pc)) => Some(ac - pc),
                                _ => None,
                        };

                self.variacao_capital_giro =
                        match(&self.capital_giro_liquido_trimestre, &capital_giro_liquido){
                                (Some(cg_atual), Some(cg_banco))=>{
                                        Some(cg_atual - cg_banco)
                                }
                                _ => None,
                        }

        }
                        
        pub async fn insert(self)-> Result<(), sqlx::Error>{
                                
                                let pool = PgPoolOptions::new()
                                .connect("postgres://postgres: @localhost/valuation")
                                .await?;

                                sqlx::query!(
                                        r#"
                                                INSERT INTO ffcf_historico
                                                        (
                                                        ano,
                                                        trimestre,

                                                        fco_acumulado,
                                                        depreciacao_acumulada,
                                                        imob_intang_acumulado,
                                                        ebt_acumulado,
                                                        ebit_acumulado,
                                                        tributo_acumulado,

                                                        tributo_trimestre,
                                                        ebit_trimestre,
                                                        fco_trimestre,
                                                        depreciacao_trimestre,
                                                        imob_intang_trimestre,
                                                        ebt_trimestre,

                                                        ativo_circulante_trimestre,
                                                        passivo_circulante_trimestre,
                                                        capital_giro_liquido_trimestre,
                                                        variacao_capital_giroiro,
                                                        
                                                        )
                                                VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
                                        
                                        "#,
                                        self.ano,
                                        self.trimestre,

                                        self.fco_acumulado,
                                        self.depreciacao_acumulada,
                                        self.imob_intang_acumulado,
                                        self.ebt_acumulado,
                                        self.ebit_acumulado,
                                        self.tributo_acumulado,

                                        self.tributo_trimestre,
                                        self.ebit_trimestre,
                                        self.fco_trimestre,
                                        self.depreciacao_trimestre,
                                        self.imob_intang_trimestre,
                                        self.ebt_trimestre,

                                        self.ativo_circulante_trimestre,
                                        self.passivo_circulante_trimestre,
                                        self.capital_giro_liquido_trimestre,
                                        self.variacao_capital_giro
                                )
                                .execute(&pool)
                                .await?;

                                Ok(())
    
                                }

        



}