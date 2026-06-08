# Visão geral do protocolo

## O que a BWB faz

BWB Digital Assets é uma plataforma regulada pela CVM que transforma participações em projetos imobiliários em tokens digitais. Um investidor acessa o portal, faz o cadastro e verificação de identidade, e pode investir em uma oferta em andamento diretamente via PIX. No final do período, recebe rendimentos — agora distribuídos diretamente na sua carteira digital.

Todas as ofertas são estruturadas sob a **Resolução 88 da CVM**, a norma brasileira que regula valores mobiliários tokenizados. Isso define o que pode e o que não pode acontecer com cada token: quem pode comprar, como as transferências funcionam e quais informações precisam estar registradas on-chain.

---

## Por que Stellar agora

A plataforma BWB opera desde 2024 na rede Base (EVM). O modelo funcionou para as primeiras ofertas — R$4M+ captados, três projetos encerrados sem inadimplência. Mas dois problemas estruturais ficaram evidentes conforme o número de investidores cresceu:

**Custo de distribuição.** Para distribuir rendimentos a 100+ investidores na rede EVM, cada lote de transações custa entre $50 e $200 em taxas. No Soroban (a plataforma de contratos inteligentes da Stellar), a mesma operação custa menos de $0,10. A diferença deixa de ser marginal quando a operação é trimestral e o ticket médio do investidor é de R$10–50K.

**Liquidação em real.** Não existe uma stablecoin de real com boa liquidez e infraestrutura de entrada/saída via PIX nas redes EVM. Na Stellar, o **BRZ** — emitido pela Transfero e lastreado 1:1 em reais brasileiros — existe nativamente. A Transfero disponibiliza uma API (BaaSic) que converte pagamentos PIX em BRZ na Stellar com latência de minutos, sem custódia intermediária adicional.

A migração para Stellar não é uma troca de blockchain por razões filosóficas: é resolver um problema real de custo operacional e liquidação em moeda local.

---

## Conformidade CVM 88

A Resolução CVM 88 estabelece que ofertas de valores mobiliários tokenizados precisam:

1. **KYC do investidor** — identidade verificada antes de qualquer aporte
2. **Categorização** — investidor Varejo, Qualificado ou Profissional define os limites de exposição
3. **Transferibilidade controlada** — tokens só podem ser transferidos entre endereços aprovados
4. **Registro da oferta** — código de autorização CVM, endereço do imóvel e parâmetros financeiros acessíveis publicamente

O contrato `kyc-whitelist` implementa os itens 1–3 diretamente no Soroban: cada endereço só aparece no registro se tiver passado pelo processo de verificação. O contrato `real-estate-token` bloqueia automaticamente qualquer tentativa de transferência para endereços fora do registro, e armazena o código de autorização CVM na configuração da oferta de forma imutável.

---

## Fluxo de investimento

### Subscrição (entrada)

```
Investidor                  BWB Backend              Stellar
    │                           │                       │
    ├─ KYC no portal ──────────►│                       │
    │                           ├─ add(address, cat) ──►│ kyc-whitelist
    │                           │                       │
    ├─ PIX (R$10.000) ─────────►│                       │
    │                           ├─ BaaSic API ──────────►│ BRZ recebido
    │                           ├─ mint(addr, 10.000) ──►│ real-estate-token
    │                           │                       │
    └──────────────────── Tokens na carteira ◄──────────┘
```

1. Investidor completa o KYC no portal (dados, documentos, biometria)
2. Backend BWB (Convex) adiciona o endereço Stellar do investidor no `kyc-whitelist` com sua categoria
3. Investidor efetua PIX com o valor do aporte
4. Transfero BaaSic converte BRL→BRZ na conta operacional BWB
5. Backend confirma o pagamento e chama `mint` no `real-estate-token`, emitindo tokens para o endereço do investidor
6. Tokens aparecem na carteira Stellar do investidor

### Distribuição de rendimentos

```
Admin BWB
    │
    ├─ Calcula rendimento proporcional (NAV × holders)
    ├─ Transfere BRZ para contrato distribution
    └─ distribution::distribute(token_contract)
           ├─ Lê todos os balances do real-estate-token
           └─ Transfere BRZ proporcional a cada holder
                    → BRZ direto na carteira de cada investidor
```

O cálculo de NAV (Valor Patrimonial Líquido) usa os dados da oferta armazenados no `real-estate-token` — TIR alvo, valor captado, prazo — para produzir um valor de referência por token a qualquer momento.

---

## Papéis no sistema

| Papel | Quem | O que pode fazer |
|---|---|---|
| **Admin** | Carteira fria BWB | Governança — propose/accept admin, pausar contratos |
| **Operator** | Privy Server Wallet (hot) | Operações diárias — adicionar/remover KYC, mintagem |
| **Investidor** | Endereço Stellar do cliente | Receber tokens, transferir entre carteiras KYC-aprovadas |

A separação Admin/Operator é deliberada: o Operator pode executar operações sem precisar da chave Admin (que fica offline). A troca de Admin usa dois passos — `propose_admin` + `accept_admin` — para evitar bloqueio acidental do contrato.

O Privy Server Wallet, que já gerencia as credenciais de acesso dos usuários no BWB atual, é estendido para gerar e custodiar o keypair Ed25519 do Operator na Stellar. Nenhuma chave privada Stellar existe no código ou em variáveis de ambiente desta base de código.

---

## Tecnologias

| Componente | Tecnologia | Por quê |
|---|---|---|
| Contratos | Rust/Soroban, SEP-0041 | Padrão de token nativo Stellar; garantia de interoperabilidade |
| Backend | Convex (TypeScript) | Já em produção no BWB; funções serverless para orquestrar minting e KYC |
| Auth + Wallets | Privy | Já em produção no BWB; gerencia keypairs Ed25519 para Operator |
| Portal do investidor | Stellar Wallets Kit | Adapter Freighter/Albedo para self-custody opcional |
| Stablecoin | BRZ (Transfero) | Lastreado 1:1 em BRL; API PIX nativa; liquidez no mercado brasileiro |
| On/off-ramp | Abroad (Stellar) | Rampa BRL↔stablecoin complementar, nativa Stellar |

---

## Estado atual

| Item | Status |
|---|---|
| Plataforma BWB (EVM) | Produção — [app.bwbi.com.br](https://app.bwbi.com.br) |
| Contrato `kyc-whitelist` | Implementado, 16 testes passando |
| Contrato `real-estate-token` | Implementado (SEP-0041 completo), 22 testes passando |
| Contrato `distribution` | Spec definida, implementação T2 |
| SDK TypeScript | Planejado T1 |
| Deploy testnet | T1 |
| Deploy mainnet | T3 |

---

## Código aberto

Os contratos Soroban são licenciados Apache 2.0. Qualquer plataforma regulada que precise de um modelo de token imobiliário com KYC on-chain e conformidade com legislação local pode usar esta base.

A lógica de negócio BWB (frontend, backend Convex, contratos EVM proprietários) permanece fechada e não faz parte deste repositório.
