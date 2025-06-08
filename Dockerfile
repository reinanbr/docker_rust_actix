FROM ubuntu:24.04

# Evita prompts interativos
ENV DEBIAN_FRONTEND=noninteractive

# Instala dependências do sistema
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Instala Rust via rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Adiciona o cargo ao PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Define o diretório do app
WORKDIR /app

# Copia o projeto
COPY . .

# Compila o projeto (pode ser usado em imagem final depois)
RUN cargo build --release

CMD ["./target/release/docker_rocket"]