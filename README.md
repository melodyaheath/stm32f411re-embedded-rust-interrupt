# stm32f411re-embedded-rust-interrupt

This is based on the [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart) project.

The code is for the stm32-f411re processor, and is meant for the nucleo-f411re board. Pressed the **user b1 button** on **Pin GPIOC 17 (PC17)** will cause the led on **Pin GPIOA 5 (PA5)** to light up. This example also illustrates how to check pin state in a loop and share that resource with the interrupt through a mutex.

I've included some resources that have helped me along the way.

![Arduino Connectors Part 1](/arduino-connectors-p1.png)

![Arduino Connectors Part 2](/arduino-connectors-p2.png)


![Nucleo F411RE Mappings](/nucleo-f411re-mappings.png)
