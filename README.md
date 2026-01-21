# 🦀 Rust Embarcado com ESP32 — FSMs, Tasks e Drivers

Este repositório contém um projeto completo de estudos em **Rust para sistemas embarcados**, utilizando **ESP32 + ESP-IDF**, com foco em **arquitetura de software**, **FSMs**, **tasks**, **abstração de drivers** e **preparação para desenvolvimento de device drivers**.

O projeto foi estruturado para ser **reutilizável**, **extensível** e **manutenível**, seguindo práticas próximas ao mercado industrial.

---

## 🎯 Objetivos

- Aprender Rust aplicado a sistemas embarcados reais
- Desenvolver Máquinas de Estados Finitos (FSM)
- Separar lógica de controle e hardware
- Criar drivers abstratos via `trait`
- Construir base para drivers Linux / SoC / FPGA
- Manter código reutilizável no futuro

---

## 🧠 Conceitos Implementados

- Rust embarcado com ESP-IDF
- FSM tradicional e FSM orientada a eventos
- Temporização por tick
- Tasks estilo RTOS
- Separação por camadas (FSM / Task / Driver)
- Abstração de hardware (HAL próprio)
- Driver desacoplado da lógica
- Projeto extra com FSM (LED + buzzer)

---

## 📁 Estrutura do Projeto

```text
src/
├── main.rs
├── tasks/
│   └── led_task.rs
├── fsm/
│   └── led_fsm.rs
└── drivers/
    └── led/
        ├── mod.rs
        ├── hw.rs
        └── esp32.rs
```

## Descrição

   - main.rs

        Inicialização dos periféricos

        Injeção do driver nas tasks

   - tasks/

        Execução contínua (loop / RTOS)

        Integra FSM + hardware

   - fsm/

        Lógica pura de estados

        Não conhece hardware

   - drivers/

        Abstração de hardware via trait

        Implementações específicas (ESP32)

## 🔁 FSM (Máquina de Estados)

A FSM controla o comportamento do sistema sem acessar hardware diretamente.

Ela retorna:

   - Some(true) → ligar saída

   - Some(false) → desligar saída

   - None → manter estado

Isso permite:

   - Testar FSM isoladamente

   - Reutilizar lógica em outro hardware

   - Usar a mesma FSM em Linux, RTOS ou FPGA

⏱ FSM Orientada a Eventos

A FSM responde a eventos:

Event::Tick

O tempo é controlado por contagem de ticks, não por delays internos.
🔌 Driver Abstrato

Drivers são definidos por contrato (trait):

   - A FSM não sabe qual hardware está sendo usado

   - Qualquer struct que implemente o trait pode ser usada

   - Facilita portabilidade e testes

## 🛠 Setup do Ambiente (Linux)
1️⃣ Instalar ESP-IDF

git clone --recursive https://github.com/espressif/esp-idf.git ~/esp-idf
cd ~/esp-idf
./install.sh

2️⃣ Ativar o ambiente (sempre que abrir um terminal)

source ~/esp-idf/export.sh

Verificar:

echo $IDF_PATH

3️⃣ Toolchain Rust para ESP32

cargo install espup
espup install
source $HOME/export-esp.sh

🚀 Build e Execução
Build

cargo +esp build

Flash / Run

cargo +esp run

Ou especificando a porta:

cargo +esp run -- --port /dev/ttyUSB0

## 📌 Projeto Extra

Foi desenvolvido um projeto adicional utilizando FSM para controlar LED e buzzer, simulando diferentes estados e padrões.

O objetivo foi exercitar:

   - Modelagem de estados

   - Transições controladas

   - Arquitetura FSM reutilizável

## 🧭 Planejamento Futuro

   - Expansão de drivers (GPIO, botão, buzzer)

   - Comunicação entre tasks

   - Eventos e filas

   - Simulação de hardware

   - Base para drivers Linux

   - Memory-mapped I/O

   - Integração com FPGA

## 🎓 Objetivo Final

Alcançar domínio técnico para desenvolver:

   - Device drivers Linux

   - Drivers para SoCs

   - Drivers para FPGA

   - Sistemas embarcados complexos

## Utilizando:

   - Rust

   - FSMs

   - Arquitetura limpa

   - Abstração correta de hardware

## obrigado!
