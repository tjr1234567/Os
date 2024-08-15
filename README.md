# Os
An Operation Systerm written by rust following the instruction url"https://os.phil-opp.com/"

    Utilizing the thumbv7em-none-eabihf, a bare metal environment having no underlying operating system.
    Adding it to the rustup by the instruct:

    rustup target add thumbv7em-none-eabihf

    And start the project with the instruct:

    cargo build --target thumbv7em-none-eabihf

OR

    Alternatively, we can compile it for the host system by passing additional linker arguments:
# Linux
    cargo rustc -- -C link-arg=-nostartfiles
# Windows
    cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
    cargo rustc -- -C link-args="-e __start -static -nostartfiles"
