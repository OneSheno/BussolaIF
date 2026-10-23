# Bússola IF

Projeto em desenvolvimento para ajudar estudantes do IFBA
a acompanhar aulas, horários, salas e frequência.

Desenvolvido como projeto de aprendizado em Rust, começando
pelo campus Santo Antônio de Jesus.

## Estado atual

Protótipo de terminal com dados de exemplo.

- Exibição de disciplina e sala.
- Cálculo de presenças e frequência.
- Validação de faltas maiores que aulas lançadas.
- Tratamento de disciplinas sem aulas lançadas.

## Planejado

- Importação de dados do SUAP.
- Agenda e calendário acadêmico.
- Contas com sincronização entre PC e celular.
- Suporte aos diferentes cursos e campi.

A integração com o SUAP ainda não foi implementada.
Projeto independente, sem vínculo oficial com o IFBA.


## Viabilidade da integração com o SUAP

Testes realizados em um projeto separado confirmaram:

- Autenticação OAuth e consulta de perfil.
- Acesso ao boletim por sessão autenticada, com extração de disciplinas e faltas.
- Leitura de locais e horários.
- Leitura de períodos acadêmicos e avaliações.

A integração ainda não foi incorporada ao Bússola IF.
A fonte de calendário testada não informa feriados e recessos.

## Como executar

Com Rust e Cargo instalados:

cargo run