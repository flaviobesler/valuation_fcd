# Valuation FCD

Uma aplicação desktop desenvolvida em **Rust** para automatizar processos de valuation fundamentalista, inicialmente com foco em **Fluxo de Caixa Descontado (DCF)** e **FCFF**.

O projeto nasceu da vontade de transformar um processo que normalmente exige bastante coleta e organização manual de dados em uma ferramenta capaz de automatizar grande parte desse trabalho.

##  Objetivo

A ideia é desenvolver uma ferramenta que permita:

* calcular FCFF e fluxo de caixa livre;
* calcular WACC;
* estimar beta utilizando dados históricos de mercado;
* armazenar e organizar dados financeiros históricos;
* automatizar a coleta de dados que podem ser obtidos de fontes externas;
* analisar múltiplas empresas em vez de realizar todo o processo manualmente para uma empresa por vez;
* futuramente incorporar outros métodos de valuation e simulações.

O projeto ainda está em desenvolvimento e algumas partes estão sendo construídas e testadas conforme o modelo de valuation evolui.

## Tecnologias

* **Rust**
* **egui / eframe**
* **PostgreSQL**
* **SQLx**
* **BigDecimal**

##  Status

**Em desenvolvimento.**

Este é um projeto pessoal e experimental, criado principalmente para desenvolver uma ferramenta de valuation e, ao mesmo tempo, aprofundar conhecimentos em Rust, bancos de dados, arquitetura de software e automação de processos.

Algumas decisões de implementação podem mudar conforme os cálculos são testados com diferentes empresas e dados históricos.

##  Aviso

Este software é uma ferramenta de estudo e análise e **não constitui recomendação de investimento**.

Os resultados dependem dos dados utilizados, das premissas adotadas e do modelo de valuation implementado.

##  Segurança

As versões oficiais do projeto são aquelas disponibilizadas neste repositório.

Versões modificadas, forks ou distribuições de terceiros não são necessariamente mantidas ou endossadas pelo autor. Consulte [`SECURITY.md`](SECURITY.md) para mais informações.
