# Guide — Project Board: Ações por Item até 12/06/2026

**Contexto:** O deadline de 12/06 é para submissão do **Interest Form** em `communityfund.stellar.org`.  
O Interest Form **não exige código funcionando** — exige uma proposta técnica convincente, tração comprovada e clareza de arquitetura. O código real vem nas Tranches após aprovação.

**Lógica de priorização usada:**
- 🔴 **Antes de 12/06** — bloqueia ou prejudica gravemente a submissão
- 🟠 **Antes de 12/06 (desejável)** — aumenta credibilidade, não é bloqueador
- 🟢 **Após grant — Tranche 1+** — implementação real, não faz sentido antecipar

---

## ⚠️ DECISÕES PENDENTES

---

### Issue #1 — Decisão: Licença MIT ou Apache 2.0?
**Prazo:** 🔴 Antes de 12/06 | **Ação estimada:** 30 minutos

**Por que urgente:**  
O Interest Form pergunta sobre open-source strategy. Responder "a definir" enfraquece a proposta. Avaliadores SCF esperam clareza aqui.

**Recomendação:**  
**Apache 2.0** — sem discussão. É a escolha padrão de projetos SCF aprovados (Tigerblocks, Neovestor, Spydra usam Apache 2.0). Protege o IP comercial da BWB contra uso sem atribuição, ao contrário da MIT. O custo de troca depois é alto (requer consenso dos contribuidores).

**Ação concreta:**  
1. CEO e CTO alinham nesta issue (comentário de aprovação)
2. Fechar issue com decisão: Apache 2.0
3. O `LICENSE` já está no repo com Apache 2.0 — nenhuma mudança técnica necessária

---

### Issue #2 — Decisão: Dev Soroban — contratar ou parceiro?
**Prazo:** 🔴 BLOQUEADOR CRÍTICO — antes de 12/06 | **Ação estimada:** 2–5 dias

**Por que urgente:**  
Este é o **único item que pode inviabilizar a submissão**. O prescreen do SCF verifica se a equipe tem capacidade técnica real para implementar Soroban/Rust. Sem um nome concreto, a proposta falha neste ponto antes de chegar aos avaliadores.

**Recomendação:**  
Não tente contratar CLT em 9 dias — foque em **freelancer ou parceria imediata**. Caminhos mais rápidos:

1. **Upwork/Toptal** — busca por "Soroban developer" ou "Stellar smart contracts Rust". Existem perfis ativos. Uma call de triagem + carta de intenção já serve para o Interest Form.
2. **Discord SCF** — peça indicação de dev Soroban disponível. A comunidade frequentemente conecta projetos com devs. Bônus: gera relacionamento com a comunidade antes da submissão.
3. **Parceria com Tigerblocks ou Neovestor** — já têm dev Soroban. Propor co-desenvolvimento ou consultoria pontual para a proposta.

**O que colocar no Interest Form:**  
"We have identified/engaged [Name], a Soroban/Rust specialist with [X] experience, to lead the Stellar integration deliverables." — Um nome com LinkedIn já muda completamente a percepção dos avaliadores.

**Ação concreta:**  
1. CEO abre processo de busca HOJE (Discord SCF + Upwork)
2. Primeira call com candidato antes de 07/06
3. Comentar nesta issue com o nome e perfil do dev identificado
4. Fechar issue antes de submeter o Interest Form

---

### Issue #3 — Decisão: Integrar Tigerblocks (KYC) ou construir próprio?
**Prazo:** 🟠 Antes de 12/06 (desejável) | **Ação estimada:** 2 horas de pesquisa

**Por que relevante agora:**  
Citar integração com projetos SCF existentes é um **diferencial positivo na avaliação**. Se a proposta mencionar Tigerblocks como parceiro/referência, demonstra conhecimento do ecossistema e maturidade.

**Recomendação:**  
Para o Interest Form, declare **intenção de integrar Tigerblocks** (não precisa estar implementado). Isso:
- Demonstra ecosystem fit
- Reduz escopo técnico percebido (menos risco para avaliadores)
- Abre porta para parceria real que pode incluir o dev Soroban necessário (Issue #2)

Para implementação real: avaliar após aprovação se a arquitetura Tigerblocks é compatível com os requisitos específicos da CVM 88. Se não for, construir próprio na Tranche 1.

**Ação concreta:**  
1. CTO pesquisa repositório público do Tigerblocks (1 hora)
2. Decisão rápida: compatível com CVM 88? sim/não
3. Comentar na issue com conclusão
4. Para o Interest Form: mencionar Tigerblocks como referência de integração planejada

---

### Issue #4 — Decisão: BRLA anchor SEP-24 ou SEP-31?
**Prazo:** 🟠 Antes de 12/06 (desejável) | **Ação estimada:** 1 hora

**Por que relevante agora:**  
O Interest Form pergunta sobre integração com stablecoin Stellar. Ter uma resposta clara (mesmo que "ambos") demonstra profundidade técnica.

**Recomendação:**  
Declare **SEP-31 como primário + SEP-24 como fallback**. Argumento para avaliadores: SEP-31 habilita investidores internacionais (USD/EUR → BRLA), que é exatamente o diferencial que Stellar agrega vs Base. SEP-24 garante a experiência para investidores brasileiros via PIX.

**Ação crítica paralela:**  
Contatar BRLA Digital para confirmar se já têm anchor Stellar ativo (testnet ou mainnet). Se sim, isso é tração concreta para mencionar no Interest Form. Se não, mencioná-los como parceiro confirmado para integração.

**Ação concreta:**  
1. CEO contata BRLA Digital (email/WhatsApp) — pergunta sobre anchor Stellar
2. CTO decide SEP-24 vs SEP-31 com base na resposta da BRLA
3. Fechar issue com decisão documentada

---

### Issue #5 — Decisão: Workspace Rust monorepo ou repos separados?
**Prazo:** 🟢 Após grant — Tranche 1 | **Ação estimada:** N/A agora

**Por que pode esperar:**  
Esta é uma decisão de arquitetura interna de desenvolvimento. Não aparece no Interest Form e não é avaliada pelos revisores SCF antes da aprovação. O scaffold atual já usa monorepo (Cargo workspace) — se o time concordar, basta manter.

**Recomendação para o futuro:**  
Manter monorepo (já implementado). Mudar para repos separados traz overhead de CI/CD sem benefício claro neste estágio.

**Ação concreta:**  
Fechar a issue com decisão: monorepo. Revisitar apenas se o dev Soroban contratado tiver preferência diferente com justificativa técnica.

---

## 📦 CONTRACTS

---

### Issue #6 — [Contracts] real-estate-token — token RWA imobiliário
**Prazo:** 🟢 Após grant — Tranche 1 (Semanas 1–4) | **Milestone:** T1

**Por que pode esperar:**  
O Interest Form não exige contrato funcionando. O scaffold Rust já está no repo (`contracts/real-estate-token/src/lib.rs`) — isso é suficiente para demonstrar seriedade técnica aos avaliadores.

**O que fazer ANTES de 12/06 (sem implementar):**  
Garantir que o arquivo `src/lib.rs` existente está claro, com comentários em inglês explicando a arquitetura. Avaliadores técnicos do SCF podem abrir o repo e ler o código. O scaffold atual já cumpre isso.

**Ação Tranche 1:**  
Implementação completa com dev Soroban contratado (Issue #2). Entregável verificável: `cargo test` passando com ≥80% de cobertura.

---

### Issue #7 — [Contracts] kyc-whitelist — KYC-gated transfers CVM 88
**Prazo:** 🟢 Após grant — Tranche 1 (Semanas 1–4) | **Milestone:** T1

**Por que pode esperar:**  
Mesmo raciocínio do Issue #6. O scaffold com os 2 testes unitários básicos (`test_add_and_verify`, `test_remove`) já demonstra que a equipe entende a lógica. É mais do que a maioria dos projetos SCF aprovados tinham no momento da submissão.

**Diferencial importante para o Interest Form:**  
Na proposta, destacar que este contrato implementa CVM 88 on-chain — isso é o coração do argumento "Stellar é CORE" e não superficial. Use a documentação do `docs/kyc-flow.md` para embasar.

**Ação Tranche 1:**  
Implementação completa + integração cross-contract com `real-estate-token`.

---

### Issue #8 — [Contracts] distribution — rendimentos programáticos
**Prazo:** 🟢 Após grant — Tranche 3 (Semanas 9–14) | **Milestone:** T3

**Por que pode esperar com tranquilidade:**  
Este contrato é Tranche 3. Não há nenhuma expectativa de estar implementado no Interest Form. O scaffold existe, o conceito está documentado — suficiente.

**O que destacar no Interest Form:**  
O argumento de custo (distribuição para 100+ holders custando fração do gas EVM) é um dos mais fortes para justificar "por que Stellar é CORE". Use este argumento no campo "Why Stellar?" do formulário — não precisa do contrato implementado para fazer o argumento.

**Ação:** Nenhuma antes de 12/06. Retomar na Tranche 3.

---

## 🔧 SDK

---

### Issue #9 — [SDK] Cliente Stellar/Horizon — TypeScript
**Prazo:** 🟢 Após grant — Tranche 2 (Semanas 5–8) | **Milestone:** T2

**Por que pode esperar:**  
O scaffold em `sdk/src/client.ts` já mostra a estrutura técnica. O Interest Form não exige SDK funcionando.

**Ação antes de 12/06 (opcional, alto impacto):**  
Se o dev Soroban ou CTO tiver 2–3 horas disponíveis, implementar o método `getAccountBalance()` no `client.ts` — é trivial com `@stellar/stellar-sdk` e faz o repo parecer mais ativo e real quando avaliadores abrirem o código.

**Ação Tranche 2:** Implementação completa de todos os módulos SDK.

---

### Issue #10 — [SDK] Integração BRLA Anchor — SEP-24/31
**Prazo:** 🟢 Após grant — Tranche 3 (Semanas 9–14) | **Milestone:** T3

**Dependência crítica:**  
Bloqueia: confirmação da BRLA Digital sobre anchor Stellar (ver Issue #4). Sem isso, não há como implementar.

**Ação antes de 12/06:** Apenas confirmar com BRLA Digital se têm ou planejam ter anchor Stellar. Resultado da conversa documenta na issue e reforça o Interest Form.

**Ação Tranche 3:** Implementação completa após confirmação da BRLA.

---

## ⚙️ BACKEND

---

### Issue #11 — [Backend] Integração Convex com Stellar
**Prazo:** 🟢 Após grant — Tranche 2 (Semanas 5–8) | **Milestone:** T2

**Por que pode esperar:**  
O time de backend da BWB já conhece Convex profundamente. Esta integração é de Tranche 2 e depende do SDK (Issue #9) estar pronto.

**O que fazer antes de 12/06:**  
No Interest Form, descrever a arquitetura Convex → Stellar SDK → Soroban com clareza. O `docs/architecture.md` já tem o diagrama — usem-no como base para o campo "Technical Architecture" do formulário.

**Ação Tranche 2:** Implementação completa das mutations e queries.

---

## 🖥️ FRONTEND

---

### Issue #12 — [Frontend] Stellar wallet adapter — portal BWB
**Prazo:** 🟢 Após grant — Tranche 3 (Semanas 9–14) | **Milestone:** T3

**Por que pode esperar:**  
Frontend é o último componente a ser integrado e está na Tranche 3. O portal atual (app.bwbi.com.br) já existe e funciona — isso é o que mostrar no vídeo demo para o SCF.

**O que fazer antes de 12/06:**  
Gravar o vídeo demo do portal **atual** funcionando (app.bwbi.com.br com uma oferta real). Não precisa mostrar Stellar wallet — mostra o produto real com tração comprovada.

**Ação Tranche 3:** Integração do Freighter/Albedo no portal existente.

---

## 🏗️ INFRA/CI

---

### Issue #13 — [Infra/CI] GitHub Actions — build e testes Soroban
**Prazo:** 🟠 Antes de 12/06 (desejável) | **Ação estimada:** 2–3 horas

**Por que vale antecipar:**  
Um badge verde de CI no README é um sinal visual imediato de qualidade para avaliadores técnicos. O arquivo `.github/workflows/ci.yml` já está no repo — falta apenas o contrato compilar para o pipeline rodar verde.

**O que fazer antes de 12/06:**  
Simplificar o `ci.yml` para rodar apenas `cargo build` nos contratos scaffold (sem testes completos). Isso garante o badge verde antes da submissão. Levar 1–2 horas com o CTO.

**Ação Tranche 1:** Pipeline completo com testes e deploy automático na testnet.

---

### Issue #14 — [Infra/CI] Scripts deploy testnet e mainnet
**Prazo:** 🟢 Após grant — Tranche 2 | **Milestone:** T2

**Por que pode esperar:**  
Scripts de deploy só fazem sentido quando os contratos estiverem implementados. O scaffold atual em `scripts/` já documenta o que será feito.

**Ação Tranche 2:** Testar e validar scripts com contratos reais na testnet.

---

## 📚 DOCS

---

### Issue #15 — [Docs] Architecture diagram — sistema completo EN
**Prazo:** 🔴 Antes de 12/06 — CRÍTICO | **Ação estimada:** 3–4 horas

**Por que urgente:**  
O diagrama de arquitetura é o coração da proposta técnica. É o primeiro artefato que avaliadores técnicos do SCF olham para entender se o projeto é real. O `docs/architecture.md` já tem diagramas em texto (ASCII) — mas o ideal para o Interest Form é um diagrama visual (imagem ou Mermaid renderizado).

**Recomendação:**  
Converter o diagrama ASCII do `architecture.md` em um diagrama visual usando:
- **Excalidraw** (gratuito, exporta PNG) — 1 hora de trabalho
- **Mermaid** (já suportado pelo GitHub) — adicionar bloco ```mermaid no .md

O PNG do diagrama pode ser incluído no próprio README.md, tornando o repo visualmente mais profissional.

**Ação concreta antes de 12/06:**  
1. CTO ou dev cria diagrama visual no Excalidraw (Base atual → Stellar futuro)
2. Exporta como PNG e adiciona em `docs/assets/architecture.png`
3. Embute no README.md com `![Architecture](docs/assets/architecture.png)`
4. Fechar issue após merge

---

### Issue #16 — [Docs] KYC flow CVM 88 — compliance on-chain EN
**Prazo:** 🔴 Antes de 12/06 — CRÍTICO | **Ação estimada:** 1 hora de revisão

**Por que urgente:**  
O campo "Why Stellar is CORE?" do Interest Form é o campo mais crítico e mais eliminatório. A resposta deve mencionar explicitamente o KYC on-chain via Soroban como implementação da CVM 88. O documento `docs/kyc-flow.md` já está completo — precisa apenas ser revisado pelo CEO/Compliance e validado como correto.

**Ação concreta antes de 12/06:**  
1. CEO/Compliance lê o `docs/kyc-flow.md` e valida a descrição da CVM 88
2. Corrige qualquer imprecisão regulatória
3. O conteúdo da seção "CVM 88 vs SEC/FCA" deve ser colado diretamente no Interest Form como argumento de equivalência regulatória
4. Fechar issue após validação

---

### Issue #17 — [Docs] SCF deliverables por tranche — relatório EN
**Prazo:** 🔴 Antes de 12/06 — CRÍTICO | **Ação estimada:** 2 horas de revisão

**Por que urgente:**  
O roadmap de 4 tranches é um campo obrigatório do Interest Form (e da submissão completa). O `docs/scf-deliverables.md` já tem toda a estrutura — precisa ser revisado pelo CEO e CTO para garantir que os valores, prazos e entregáveis estão alinhados com a realidade da equipe.

**Pontos de atenção para revisão:**  
- Os valores por tranche ($15K, $30K, $45K, $60K) estão corretos e defensáveis?
- Os prazos (semanas 1–4, 5–8, etc.) são realistas dado o dev Soroban a contratar?
- Os entregáveis são verificáveis por um avaliador externo sem follow-up?

**Ação concreta antes de 12/06:**  
1. CEO e CTO revisam o documento juntos (1 reunião de 1 hora)
2. Ajustam valores e prazos conforme necessário
3. Copiam o conteúdo das tranches diretamente para o Interest Form
4. Fechar issue após validação

---

## 📊 RESUMO EXECUTIVO

### O que fazer antes de 12/06 (em ordem de prioridade)

| # | Ação | Responsável | Prazo | Impacto |
|---|---|---|---|---|
| 1 | Identificar dev Soroban (nome + LinkedIn) | CEO | 07/06 | 🔴 BLOQUEADOR |
| 2 | Entrar no Discord SCF e pedir referral | CEO | HOJE | 🔴 BLOQUEADOR |
| 3 | Validar `docs/kyc-flow.md` com Compliance | CEO/Compliance | 06/06 | 🔴 Crítico |
| 4 | Revisar e aprovar `docs/scf-deliverables.md` | CEO + CTO | 07/06 | 🔴 Crítico |
| 5 | Decidir licença Apache 2.0 (fechar issue #1) | CEO + CTO | 05/06 | 🔴 Crítico |
| 6 | Criar diagrama visual de arquitetura (PNG) | CTO | 08/06 | 🟠 Alto |
| 7 | Contatar BRLA Digital sobre anchor Stellar | CEO | 06/06 | 🟠 Alto |
| 8 | Simplificar CI para badge verde no GitHub | CTO | 08/06 | 🟠 Alto |
| 9 | Pesquisar Tigerblocks para citar na proposta | CTO | 07/06 | 🟠 Médio |
| 10 | Gravar vídeo demo do portal atual (2–3 min EN) | Equipe | 10/06 | 🟠 Alto |
| 11 | Submeter Interest Form | CEO | 12/06 | ⭐ DEADLINE |

### Itens que ficam para após o grant (Tranche 1+)

| Issue | Item | Tranche |
|---|---|---|
| #5 | Decisão monorepo (já resolvido) | — |
| #6 | Implementação real-estate-token | T1 |
| #7 | Implementação kyc-whitelist completa | T1 |
| #8 | Implementação distribution | T3 |
| #9 | SDK TypeScript completo | T2 |
| #10 | BRLA anchor SEP-24/31 | T3 |
| #11 | Integração Convex ↔ Stellar | T2 |
| #12 | Stellar wallet adapter no portal | T3 |
| #13 | CI completo com deploy testnet | T1 |
| #14 | Scripts de deploy validados | T2 |

---

> **Conclusão:** Dos 17 itens do board, **apenas 6 precisam de ação real antes de 12/06** (issues #1, #2, #4, #15, #16, #17). O restante é execução pós-aprovação. O foco total da equipe até o deadline deve ser: **contratar o dev Soroban, obter o referral code e preencher o Interest Form com os documentos já produzidos.**
