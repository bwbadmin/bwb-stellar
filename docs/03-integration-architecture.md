# Arquitetura de integração — BWB + Stellar

Este documento descreve como a plataforma BWB existente se conecta à rede Stellar. A integração adiciona Stellar como camada de liquidação e emissão de tokens para novos produtos, operando em paralelo com a infraestrutura EVM atual.

---

## Visão geral do sistema

```
┌─────────────────────────────────────────────────────────────┐
│                    INVESTIDOR (Brasil)                        │
│           PIX → BRZ (Transfero) · Freighter/Albedo           │
└──────────────────┬──────────────────────────────────────────┘
                   │ HTTPS
                   ▼
┌─────────────────────────────────────────────────────────────┐
│              BWB Platform — app.bwbi.com.br                  │
│         React 19 + Privy (auth) + Stellar Wallets Kit        │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Convex Backend                             │
│           TypeScript serverless — real-time                  │
│     IRampProvider → TransferoRampProviderImpl                │
└──────────┬──────────────────────────────────────────────────┘
           │ BWB Stellar SDK (sdk/src/)
           ▼
┌─────────────────────────────────────────────────────────────┐
│              Soroban RPC + Horizon API                       │
│      Futurenet (dev) · Testnet · Mainnet                     │
└──────────┬────────────────┬───────────────────────────────┬─┘
           ▼                ▼                               ▼
  real-estate-token    kyc-whitelist              distribution
  (um por oferta)      (registro compartilhado)   (BRZ por holder)
```

---

## Componentes principais

### 1. Privy Server Wallet — keypair do Operator

A integração Privy atual gerencia um Platform Server Wallet para operações EVM. Para Stellar, o mesmo Privy Server Wallet gerencia um keypair Ed25519 que atua como **Operator** nos contratos Soroban.

O Operator é a carteira quente que:
- Chama `kyc-whitelist::add` e `remove` quando eventos KYC chegam
- Chama `real-estate-token::mint` após confirmação de pagamento
- Dispara `distribution::distribute` nos eventos de rendimento

Uma carteira fria separada (hardware key) mantém o papel Admin — controla atualizações de contrato e transferência de administração.

Esse modelo espelha a arquitetura EVM existente (`PlatformServerWallet` + relay). Nenhum novo modelo de custódia é necessário.

### 2. Transfero BaaSic API — rampa PIX → BRZ

Transfero é a rampa principal para Stellar. A API BaaSic converte pagamentos PIX em BRZ (a stablecoin de real da Transfero, lastreada 1:1 no ativo em reais) diretamente na rede Stellar, sem custódia intermediária adicional.

```
Investidor → PIX (R$X)
  │
  └── Transfero BaaSic API
        BRL → BRZ (Stellar, lastro 1:1 em reais)
        BRZ creditado no endereço operacional BWB
          │
          └── Webhook → Convex workpool
                pagamento confirmado → mint de tokens
```

- **BRZ** é a stablecoin de BRL da Transfero na Stellar, com lastro 1:1 e resgate via PIX
- O workpool Convex trata webhooks da Transfero com retry automático
- Acesso à API BaaSic via contrato B2B — sandbox disponível para integração

### 3. Abroad — rampa complementar

O [Abroad](https://www.abroad.finance/) é uma rampa BRL↔stablecoin nativa na Stellar. Utilizada como alternativa e para investidores que preferem sacar BRZ de volta para BRL via rota diferente da Transfero.

### 4. Stellar Wallets Kit — portal do investidor

Para investidores que preferem usar sua própria carteira Stellar em vez da carteira gerenciada pela plataforma, o Stellar Wallets Kit expõe um adapter Freighter/Albedo no portal BWB.

- Investidor conecta a própria carteira Stellar
- Carteira adicionada ao `kyc-whitelist` após verificação
- Tokens emitidos diretamente para o endereço self-custodied

### 5. BWB Stellar SDK (`sdk/src/`)

O SDK é a camada TypeScript entre o backend Convex e os contratos Soroban.

| Módulo | Responsabilidade |
|---|---|
| `client.ts` | Conexão Soroban RPC com fallback multi-endpoint |
| `token.ts` | `mint`, `transfer`, `balance`, `total_supply`, `get_offering`, `nav` |
| `kyc.ts` | `add`, `remove`, `is_ok`, `get_entry` |
| `anchor.ts` | Helpers para integração com Transfero (BRZ) e Abroad |

O SDK produz XDRs não assinados para a maioria das operações — o backend Convex os assina com o keypair do Operator via Privy Server Wallet.

---

## Fluxo completo — subscrição de investidor

```
1.  Investidor completa KYC no portal BWB (documentos, biometria)
2.  KYC aprovado → mutation Convex dispara
3.  SDK: kyc-whitelist::add(stellar_address, category)
         assinado pelo keypair Operator (Privy Server Wallet)
4.  Investidor seleciona oferta, confirma valor de aporte
5.  Pagamento PIX iniciado → Transfero BaaSic API
6.  Webhook Transfero → Convex workpool
         pagamento confirmado (BRL recebido)
7.  SDK: real-estate-token::mint(stellar_address, token_amount)
         [interno] kyc-whitelist::is_ok verificado → prossegue
8.  Evento Stellar → Horizon indexer → atualização de saldo no frontend
9.  Investidor vê saldo de tokens no portal BWB
```

---

## Fluxo completo — distribuição de rendimentos

```
1.  Oferta vence / evento de rendimento trimestral
2.  Time financeiro aprova valor de distribuição (BRZ)
3.  Action agendada Convex dispara
4.  SDK: distribution::distribute(token_contract, brz_amount)
         contrato lê total_supply
         itera todos os holders
         transfere BRZ proporcional para cada endereço
5.  Log de distribuição on-chain (contagem, valor, timestamp)
6.  Investidores veem saldo BRZ aumentado em suas carteiras Stellar
7.  Opcional: investidores sacam BRZ → BRL via Transfero (PIX)
```

---

## Configuração de ambiente

| Variável | Descrição |
|---|---|
| `STELLAR_NETWORK` | `testnet` ou `mainnet` |
| `SOROBAN_RPC_URL` | Endpoint RPC principal (com lista de fallback) |
| `HORIZON_URL` | URL da API Horizon |
| `KYC_CONTRACT_ID` | Endereço do contrato kyc-whitelist deployado |
| `OPERATOR_PUBLIC_KEY` | Chave pública Stellar do Operator (Ed25519) |

Chaves privadas nunca são armazenadas neste repositório. A chave privada do Operator é gerenciada exclusivamente pelo Privy Server Wallet no backend privado BWB.

---

## Separação de responsabilidades

```
bwb-stellar (este repositório — público)
  ├── contracts/   Apache 2.0 — código aberto permanente
  ├── sdk/         Apache 2.0 — código aberto permanente
  └── docs/        Apache 2.0 — código aberto permanente

bwb-tokenization (repositório privado)
  ├── frontend/    App React — proprietário
  ├── backend/     Backend Convex — proprietário
  └── contracts/   Contratos EVM — proprietário
```

O backend privado consome o SDK público como dependência. Os contratos Soroban não têm dependência do backend privado — podem ser usados de forma independente por qualquer projeto na Stellar.
