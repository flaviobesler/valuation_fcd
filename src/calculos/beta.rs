
pub async fn indices(&self, pool:sqlx::PgPool){
        //fazendo a media para calcular o desvio do indice
        let mut soma_indice = 
                sqlx::query_scalar!(
                r#"
                SELECT COALESCE(SUM(variacao_mes),0)
                FROM (
                        SELECT variacao_mes
                        FROM historico_indice
                        WHERE ticket = $1
                       )
                "#,
                self.ticket
                )
                .fetch_all(&pool)
                .await?;
        
        let mut contagem_indice = 
                sqlx::query_scalar!(
                r#"
                SELECT COUNT(variacao_mes)
                        FROM historico_indice
                        WHERE ticket = $1
                "#
                ,self.ticket
                )
                .fetch_one(&pool)
                .await?;
                
               
        let retorno_indice = match(&soma_indice, &contagem_indice){
                (Some(soma_indice), Some(contagem_indice)) =>{
                        Some(soma_indice/contagem_indice)
                }_ => None,
                
        };

        //encontrando o desvio do indice
        let indice_banco = 
                sqlx::query_scalar!(
                        r#"
                        SELECT variacao_mes
                        FROM historico_indice
                                WHERE ticket = $1
                                ORDER BY ultimo_pregao
                        "#,
                        self.ticket
                )
                .fetch_all(&pool)
                .await?;

        let mut indice_lista = Vec::new();


        if let Some(media)= retorno_indice{
                for valor in &indice_banco{
                let mut desvio_indice = valor - &retorno_indice;

                        indice_lista.push(desvio_indice)                ;
                };                
                        
        };

        //é necessario criar uma nova lista com todos os valor dos desvios ao quadrado
        //para soma-los e criar a variancia da formula do beta
        let variancia_lista  = Vec::new() ;

        for valor in indice_lista{
                let mut calculo_variancia = valor*valor;

                variancia_lista.push(calculo_variancia)
                        
        };

        let mut soma = BigDecimal::from(0);
        for i in 0..variancia_lista.len(){
                soma += variancia_lista[i];
        }
        let mut variancia  = soma /  contagem_indice;
        let desvio_padrão = variancia**0.5;

}

pub async fn beta(&self, pool:sqlx::PgPool) -> Result<(), Box<dyn Error>>{

        //usando a variação mensal para descobrir o desvio mensal da ação
        let desvio_ativo = 
        sqlx::query_scalar!(
                r#"
                SELECT variacao_mes
                FROM dados historico
                        WHERE ticket = $1
                        ORDER BY ultimo_pregao
                "#,
                )
                .fetch_all(&pool)
                .await?;
        let retorno = risco_retorno(rm_mensal);

        let desvio_lista_ativo = Vec::new();

        for valor in desvio_ativo{
                let desvio = desvio_ativo - retorno;
                desvio_lista_ativo.push(desvio)
        }

        //resgatando valores da função indice para calcular
        let indice =  indices(indice_lista);
        let quantidade_indice = indice(contagem_indice);
        let variancia = indices(variancia);

        let covariancia_lista = Vec::new();

        if desvio_ativo.len() != indice.len(){
                return Err("vetores de tamanhos diferentes".into());
        }

        let result = desvio_ativo
                .iter()
                .zip(indice.iter())
                .map(|(a,b)|a*b)
                .collect();

        let mut soma = 0;
        for i in 0..result.len(){
                let mut covariancia = (soma += result[i])/media;
                
        }

        let beta = covariancia / variancia;
        
}


pub async fn risco_retorno(&self, pool: &sqlx::PgPool, ticket: String){
        //encontrado os dados da formula de custo de capital proprio
        //capm(e) = Rf + βa × (Rm − Rf)
        //o beta será feito em outra função
        let rf = match(&self.taxa_titulo, &self.inflacao){
                (Some(taxa), Some(inflacao))=>{
                        Some(((1+taxa)/(1+inflacao))-1)
                }
        };

        let e_rm = 
                sqlx::query_scalar!(
                                r#"
                                SELECT COALESCE(SUM(valor),0)
                                        FROM( SELECT valor
                                              FROM dados_historicos
                                              WHERE ticket = $1)
                        "#,ticket
                )
                .fetch_all(&pool)
                .await?;
        let quantidade = 
                sqlx::query_scalar!(
                        r#"
                                SELECT COUNT(valor)
                               FROM dados_historicos
                               WHERE ticket = $1
                       "#,
                        ticket
                )
                .fetch_one(&pool)
                .await?;

        let rm_mensal = match(&e_rm, &quantidade){
                (Some(rm), Some(n))=>{
                        Some(rm/n)
                }_=>None,
        };
        let rm_anual = ((1+rm_mensal)**12)-1;


        let beta_calculado = beta(beta);

        let capm = rf + beta_calculado * (rm_anual- rf);

        
}


