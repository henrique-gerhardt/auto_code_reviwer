# Revisão de Código com OpenAI GPT-4o
 
Este é um projeto em Rust que extrai as alterações de arquivos de um Pull Request (PR) do GitHub ou de um Merge Request (MR) do GitLab e formata essas alterações em um texto contínuo otimizado para análise por modelos de linguagem de inteligência artificial. Com objetivo de enviar prompts para a API do OpenAI GPT-4o para revisão de código automático.

## Funcionalidades

- Suporte para Pull Requests do GitHub.
- Suporte para Merge Requests do GitLab.
- Configuração de tokens de autenticação via linha de comando.
- Formatação das alterações em um texto contínuo com blocos de código.

## Dependências

- `reqwest`: Para fazer requisições HTTP.
- `serde`: Para serialização e deserialização de dados.
- `serde_json`: Para manipulação de dados JSON.
- `tokio`: Para programação assíncrona.
- `clap`: Para análise de argumentos da linha de comando.
- `dotenv`: Para gerenciar variáveis de ambiente.

## Instalação

1. Clone o repositório:

    ```sh
    git clone https://github.com/seu-usuario/git-pr-diff-fetcher.git
    cd git-pr-diff-fetcher
    ```

2. Configure o projeto:

    ```sh
    cargo build
    ```

3. Crie um arquivo `.env` na raiz do projeto para armazenar os tokens:

    ```sh
    touch .env
    ```

## Uso

### Configuração dos Tokens

1. Configure o token do GitLab:

    ```sh
    cargo run -- config git-lab-token {seu_token_gitlab}
    ```

2. Configure o token do GitHub:

    ```sh
    cargo run -- config git-hub-token {seu_token_github}
    ```

3. Configure o token da OpenAi:

    ```sh
    cargo run -- config open-ai-token {seu_token_openai}
    ```
### Executando o Programa

1. Para rodar o programa com um link de PR do GitHub ou MR do GitLab:

    ```sh
    cargo run -- run {link_do_pr_ou_mr}
    ```

## Contribuição

1. Fork este repositório.
2. Crie um branch para sua feature (`git checkout -b feature/nova-feature`).
3. Commit suas alterações (`git commit -am 'Adiciona nova feature'`).
4. Envie para o branch (`git push origin feature/nova-feature`).
5. Abra um Pull Request.

## Licença

Este projeto está licenciado sob a Licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

