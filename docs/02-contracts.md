# Contratos Soroban — Especificação

Três contratos Soroban implementam a lógica on-chain da BWB. Todos em Rust, licença Apache 2.0, em `contracts/`.

---

## Contrato 1 — `kyc-whitelist`

**Path:** `contracts/kyc-whitelist/src/lib.rs`  
**Função:** Registro de investidores aprovados — base para a conformidade CVM 88  
**Testes:** 16 passando

### Storage

| Chave | Tipo | Tier | Descrição |
|---|---|---|---|
| `Admin` | `Address` | Instance | Administrador do contrato (carteira fria) |
| `PendingAdmin` | `Address` | Instance | Successor proposto (transferência em dois passos) |
| `Operator` | `Address` | Instance | Carteira quente para operações diárias de KYC |
| `Entry(Address)` | `WhitelistEntry` | Persistent | Registro de aprovação por investidor |

Storage persistente para entradas de investidores garante que os dados sobrevivam à compactação de ledgers. Storage de instância para configuração de governança (admin, operator).

### Tipos de dados

```rust
pub enum InvestorCategory {
    Retail,       // Varejo — limites base CVM 88
    Qualified,    // Qualificado — patrimônio financeiro R$1M+
    Professional, // Profissional — R$10M+ / institucional
}

pub struct WhitelistEntry {
    pub investor_category: InvestorCategory,
    pub approved_at: u64,    // timestamp do ledger de aprovação
    pub approved_by: Address, // endereço que executou a aprovação
}
```

### Funções

**Inicialização**

| Função | Auth | Descrição |
|---|---|---|
| `initialize(admin)` | admin | Deploy e definição do admin. Pânico se chamado duas vezes. |

**Governança (admin)**

| Função | Auth | Descrição |
|---|---|---|
| `propose_admin(new_admin)` | admin | Passo 1 da transferência — propõe successor |
| `accept_admin()` | pending admin | Passo 2 — conclui transferência; admin anterior perde acesso |
| `set_operator(operator)` | admin | Define a carteira quente para operações de KYC |
| `remove_operator()` | admin | Revoga o operator — só admin pode operar após isso |

**Operações KYC (admin ou operator)**

| Função | Auth | Descrição |
|---|---|---|
| `add(caller, address, category)` | admin ou operator | Adiciona investidor com categoria CVM |
| `remove(caller, address)` | admin ou operator | Remove investidor — pânico se não existir |

**Leitura (sem auth)**

| Função | Retorna | Descrição |
|---|---|---|
| `is_ok(address)` | `bool` | Verifica aprovação — chamado pelo `real-estate-token` em toda transferência |
| `get_entry(address)` | `Option<WhitelistEntry>` | Detalhes completos ou `None` |
| `get_admin()` | `Address` | Admin atual |
| `get_operator()` | `Option<Address>` | Operator atual ou `None` |

**Gerenciamento de TTL**

| Função | Auth | Descrição |
|---|---|---|
| `extend_ttl()` | nenhuma | Estende TTL do storage de instância (heartbeat periódico) |
| `extend_entry_ttl(address)` | nenhuma | Estende TTL da entrada de um investidor específico |

### Eventos

| Evento | Payload | Quando |
|---|---|---|
| `kyc_add` | `address` | Investidor aprovado |
| `kyc_rm` | `address` | Investidor removido |
| `adm_prop` | `new_admin` | Transferência de admin proposta |
| `adm_new` | `new_admin` | Transferência de admin concluída |
| `op_set` | `operator` | Operator definido |

### Invariantes

- Apenas `Admin` ou `Operator` podem modificar o whitelist
- `Admin` está sempre definido — o contrato não funciona sem ele
- `Operator` é opcional — se não definido, só o admin opera
- Transferência de admin requer dois passos (propose + accept) — evita bloqueio acidental
- `is_ok` é leitura pura — sem auth, chamável por qualquer contrato sem custo adicional
- Remoção de investidor preserva o histórico de eventos (trilha de auditoria intacta)

---

## Contrato 2 — `real-estate-token`

**Path:** `contracts/real-estate-token/src/lib.rs`  
**Função:** Token de oferta imobiliária — SEP-0041 completo com KYC gate e conformidade CVM 88  
**Testes:** 22 passando (incluindo testes cross-contract com kyc-whitelist)

### Storage

| Chave | Tipo | Tier | Descrição |
|---|---|---|---|
| `Admin` | `Address` | Instance | Administrador do contrato |
| `PendingAdmin` | `Address` | Instance | Successor proposto |
| `Operator` | `Address` | Instance | Carteira quente para minting |
| `KycContract` | `Address` | Instance | Endereço do contrato kyc-whitelist |
| `TotalSupply` | `i128` | Instance | Oferta total de tokens |
| `Name` | `String` | Instance | Nome do token (ex: "BWB ARTP-HS Token") |
| `Symbol` | `String` | Instance | Símbolo do token (ex: "ARTP") |
| `Metadata` | `OfferingMetadata` | Instance | Dados imutáveis da oferta |
| `Paused` | `bool` | Instance | Flag de pausa de emergência |
| `Balance(Address)` | `i128` | Persistent | Saldo por investidor |
| `Allowance(AllowanceKey)` | `AllowanceValue` | Temporary | Autorizações de gasto (SEP-0041) |

### Tipos de dados

```rust
pub struct OfferingMetadata {
    pub offering_id: String,        // ID interno BWB (ex: "ARTP-HS")
    pub property_address: String,   // Endereço do imóvel no Brasil
    pub total_raise: i128,          // Captação total em centavos de BRL
    pub target_irr_bps: u32,        // TIR alvo em basis points (2080 = 20,80%)
    pub maturity_date: u64,         // Timestamp Unix do vencimento
    pub cvm_authorization: String,  // Código de autorização CVM
}

pub struct AllowanceKey {
    pub from: Address,
    pub spender: Address,
}

pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32, // Autorização expira neste ledger
}
```

### Funções SEP-0041 (padrão de token Stellar)

SEP-0041 é o padrão de token do Soroban, equivalente ao ERC-20 no Ethereum. Todos os wallets Stellar e ferramentas do ecossistema reconhecem contratos que implementam esta interface.

| Função | Auth | Descrição |
|---|---|---|
| `balance(id) → i128` | nenhuma | Saldo do endereço |
| `transfer(from, to, amount)` | from | Transferência — destinatário deve ser KYC-aprovado |
| `transfer_from(spender, from, to, amount)` | spender | Transferência com allowance prévia |
| `approve(from, spender, amount, expiration_ledger)` | from | Autoriza gasto em nome do holder |
| `allowance(from, spender) → i128` | nenhuma | Consulta allowance atual |
| `burn(from, amount)` | from | Queima tokens do próprio saldo |
| `burn_from(spender, from, amount)` | spender | Queima tokens com allowance prévia |
| `decimals() → u32` | nenhuma | Retorna `7` (padrão Stellar) |
| `name() → String` | nenhuma | Nome do token |
| `symbol() → String` | nenhuma | Símbolo do token |

### Funções específicas BWB

| Função | Auth | Descrição |
|---|---|---|
| `initialize(admin, operator, kyc_contract, name, symbol, metadata)` | admin | Deploy com detalhes da oferta |
| `mint(caller, to, amount)` | admin ou operator | Emite tokens para investidor KYC-aprovado |
| `total_supply() → i128` | nenhuma | Total de tokens emitidos |
| `get_offering() → OfferingMetadata` | nenhuma | Dados da oferta on-chain |
| `get_admin() → Address` | nenhuma | Admin atual |
| `get_operator() → Option<Address>` | nenhuma | Operator atual |
| `is_paused() → bool` | nenhuma | Estado de pausa |
| `nav() → i128` | nenhuma | NAV por token em centavos de BRL (`total_raise / total_supply`) |
| `extend_ttl()` | nenhuma | Estende TTL do storage de instância |
| `extend_balance_ttl(address)` | nenhuma | Estende TTL do saldo de um investidor |

### Governança (admin)

| Função | Auth | Descrição |
|---|---|---|
| `propose_admin(new_admin)` | admin | Propõe novo admin |
| `accept_admin()` | pending admin | Conclui transferência |
| `set_operator(operator)` | admin | Define carteira quente |
| `pause()` | admin | Pausa transferências de emergência |
| `unpause()` | admin | Retoma operação normal |

### Conformidade CVM 88

Toda função que movimenta tokens (`mint`, `transfer`, `transfer_from`, `burn_from`) chama internamente `kyc-whitelist::is_ok(address)` antes de executar. Se o endereço não está no whitelist, a transação é rejeitada:

```
Transfer rejected: recipient not KYC-approved (CVM 88)
```

A verificação é cross-contract — o `real-estate-token` invoca o `kyc-whitelist` diretamente via `env.invoke_contract`. Não há como bypassar esse check pelo frontend.

### Eventos

| Evento | Payload | Quando |
|---|---|---|
| `mint` | `(to, amount)` | Tokens emitidos |
| `transfer` | `(from, to, amount)` | Transferência executada |
| `approve` | `(from, spender, amount)` | Allowance criada |
| `burn` | `(from, amount)` | Tokens queimados |

### Invariantes

- `total_supply` = soma de todos os `Balance` em todos os momentos
- Nenhum saldo pode ser negativo (verificado antes de qualquer dedução)
- Apenas endereços KYC-aprovados podem receber tokens
- `metadata` é definida na inicialização e imutável depois
- `decimals` sempre retorna 7 (padrão Stellar)
- Allowances expiram no ledger definido — não acumulam indefinidamente

---

## Contrato 3 — `distribution`

**Path:** `contracts/distribution/src/lib.rs`  
**Função:** Distribuição proporcional de rendimentos em BRZ a todos os holders  
**Status:** Spec definida, implementação T2

### Design

Cada ciclo de distribuição:

1. Admin chama `distribute(token_contract, amount_brz)` com o total de BRZ a distribuir
2. Contrato lê `total_supply` do `real-estate-token`
3. Para cada holder: `participação = amount_brz × balance / total_supply`
4. Transfere BRZ da reserva de distribuição para cada endereço Stellar do holder
5. Registra a distribuição on-chain (contagem, valor, timestamp)

Holders não precisam executar nenhuma transação para receber — o BRZ chega diretamente no endereço Stellar registrado.

### Por que isso importa

Na rede EVM atual, distribuir para 100 holders custa $50–200 em taxas por lote. No Soroban, menos de $0,10. Essa diferença torna distribuições trimestrais economicamente viáveis no ticket médio em que a BWB opera (R$10–50K por investidor).

---

## Interações entre contratos

```
Operator (Privy Server Wallet)
  │
  ├── kyc-whitelist::add(investor_address, category)
  │
  └── real-estate-token::mint(caller, investor_address, amount)
            │
            └── [interno] kyc-whitelist::is_ok(investor_address)
                         true  → mint executado
                         false → transação rejeitada
```

```
distribution::distribute(token_contract, brz_amount)
  │
  ├── real-estate-token::total_supply()
  ├── para cada holder: real-estate-token::balance(address)
  └── BRZ::transfer(distribution_contract → holder)
```

---

## Segurança

- Todas as funções de escrita requerem `require_auth()` — nenhuma operação sem assinatura Stellar válida
- Pausa de emergência (`Paused`) bloqueia transferências sem afetar saldos — dados intactos
- Revogação de KYC é imediata — investidor removido não pode receber tokens no mesmo ledger
- Soroban storage tiers evitam expiração silenciosa de dados: balances em Persistent, allowances em Temporary
- Auditoria de segurança formal planejada antes do deploy mainnet (ver [scf-deliverables.md](scf-deliverables.md))
