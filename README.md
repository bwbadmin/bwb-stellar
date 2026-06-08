# BWB Real Estate on Stellar

> **Tokenização imobiliária regulada no Brasil, liquidada na rede Stellar.**

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Regulation](https://img.shields.io/badge/CVM%20Resolution%2088-Authorized-orange)](https://bwbi.com.br)
[![Tests](https://img.shields.io/badge/Tests-38%20passing-brightgreen)](contracts/)

---

## O que é

BWB Digital Assets é uma plataforma de investimento autorizada pela CVM (Resolução 88) que tokeniza imóveis brasileiros. Investidores aportam via PIX e recebem tokens que representam participação em ofertas imobiliárias reguladas — com rendimentos distribuídos diretamente em carteira.

Este repositório contém os contratos Soroban e o SDK TypeScript da integração com a rede Stellar.

### Histórico de ofertas

| Oferta | Captação | TIR alvo | Status |
|---|---|---|---|
| ARTP-HS | R$2,5M | 26,8% a.a. | Encerrada |
| HAUS-06 | R$1,18M | 20,5% a.a. | Encerrada |
| ARTP-DT | R$1,5M | 20,0% a.a. | Encerrada |

- **R$4M+** captados em 2025
- **Zero inadimplências**, zero reestruturações
- **7+ parceiros institucionais** (incorporadoras e originadores)
- **CVM Resolução 88** — equivalente à regulação de valores mobiliários tokenizados

---

## Por que Stellar

A plataforma BWB opera hoje na rede Base (EVM). Dois problemas concretos motivaram a migração para Stellar:

**Custo de distribuição de rendimentos.** Distribuir rendimentos trimestrais para 100+ investidores na EVM custa entre $50–200 em taxas de rede por lote. No Soroban, a mesma operação custa menos de $0,10. Isso viabiliza distribuições frequentes sem corroer o rendimento dos investidores.

**Liquidação em BRL.** Na rede Stellar, o BRZ — stablecoin da Transfero lastreado 1:1 em reais — existe nativamente. A integração com a API BaaSic da Transfero permite que um pagamento via PIX chegue como BRZ em carteira Stellar sem bridges ou custódia intermediária.

---

## Como funciona

```
Investidor (Brasil)
  │
  ├── 1. KYC aprovado na plataforma BWB
  ├── 2. Endereço Stellar adicionado ao contrato kyc-whitelist
  ├── 3. Pagamento via PIX → Transfero BaaSic API
  │         BRL → BRZ (Stellar, lastreado 1:1 em reais)
  ├── 4. Confirmação do pagamento → mint de tokens
  │         real-estate-token::mint(endereço, quantidade)
  └── 5. Tokens aparecem na carteira Stellar do investidor

Distribuição de rendimentos (trimestral)
  │
  └── distribution::distribute(contrato_token, valor_brz)
        proporcional ao saldo de cada holder → BRZ direto em carteira
```

---

## Contratos Soroban

Três contratos implementam a lógica on-chain. Todos em Rust, licença Apache 2.0.

| Contrato | Função | Testes |
|---|---|---|
| `kyc-whitelist` | Registro de investidores aprovados — CVM 88 | 16 ✅ |
| `real-estate-token` | Token da oferta — SEP-0041 completo com KYC gate | 22 ✅ |
| `distribution` | Distribuição proporcional de BRZ aos holders | T2 |

Veja [docs/02-contracts.md](docs/02-contracts.md) para a especificação completa de cada contrato.

---

## Integrações

| Componente | Papel na plataforma |
|---|---|
| [Privy](https://www.privy.io/) | Auth e gerenciamento de keypairs Ed25519 — já em produção na BWB, estendido para Stellar |
| [Stellar Wallets Kit](https://stellarwalletskit.dev/) | Adapter Freighter/Albedo para o portal do investidor |
| [Abroad](https://www.abroad.finance/) | Rampa BRL ↔ stablecoin complementar para investidores brasileiros |
| Transfero BaaSic | PIX → BRZ — liquidação nativa em Stellar |

---

## Estrutura do repositório

```
bwb-stellar/
├── contracts/
│   ├── kyc-whitelist/       # Registro KYC on-chain — CVM 88
│   ├── real-estate-token/   # Token da oferta — SEP-0041
│   └── distribution/        # Distribuição de rendimentos em BRZ
├── sdk/                     # TypeScript — cliente dos contratos
├── scripts/                 # Deploy testnet + mainnet
├── docs/                    # Documentação técnica
└── audit/                   # Relatórios de auditoria
```

---

## Documentação

| Documento | Conteúdo |
|---|---|
| [docs/01-protocol-overview.md](docs/01-protocol-overview.md) | Visão geral do protocolo — fluxo de investimento, conformidade CVM 88 |
| [docs/02-contracts.md](docs/02-contracts.md) | Especificação dos contratos — funções, storage, invariants |
| [docs/03-integration-architecture.md](docs/03-integration-architecture.md) | Arquitetura de integração — Transfero, Privy, Soroban |
| [docs/scf-deliverables.md](docs/scf-deliverables.md) | Roadmap e entregas por tranche |

---

## Código aberto

Todos os contratos Soroban em `contracts/` são licenciados **Apache 2.0** e permanecerão abertos. O código da aplicação BWB (frontend React, backend Convex, contratos EVM) é proprietário e não faz parte deste repositório.

---

## Setup para desenvolvedores

```bash
git clone https://github.com/bwbadmin/bwb-stellar.git
cd bwb-stellar

# Rodar testes dos contratos
cd contracts/kyc-whitelist && cargo test
cd contracts/real-estate-token && cargo test

# SDK TypeScript
cd sdk && npm install && npm test
```

---

## Licença

Apache 2.0 — veja [LICENSE](LICENSE)

---

## Contato

- **Website:** [bwbi.com.br](https://bwbi.com.br)
- **Plataforma:** [app.bwbi.com.br](https://app.bwbi.com.br)
- **Email:** contato@bwbi.com.br
- **Localização:** Jaraguá do Sul, SC, Brasil
