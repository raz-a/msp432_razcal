/* MSP432P401R Memory Layout */
MEMORY {
    FLASH (RX): ORIGIN = 0, LENGTH = 256K
    SRAM (RWX): ORIGIN = 0x20000000, LENGTH = 64K
}

/* Entry Point: Reset Handler */
ENTRY(reset);

EXTERN(RESET_VECTOR);
EXTERN(EXCEPTION_VECTORS);
EXTERN(INTERRUPT_VECTORS);

/* Sections */
SECTIONS {
    .vector_table ORIGIN(FLASH) : {
        /* Stack Pointer */
        LONG(ORIGIN(SRAM) + LENGTH(SRAM));

        /* Reset Vector */
        KEEP(*(.vector_table.reset));

        /* Exceptions */
        KEEP(*(.vector_table.exceptions));

        /* Interrupts */
        KEEP(*(.vector_table.interrupts));

    } > FLASH

    .text : {
        *(.text .text.*);
    } > FLASH

    .rodata : {
        *(.rodata .rodata.*);
    } > FLASH

    .bss : {
        __bss_start = .;
        *(.bss .bss.*);
        __bss_end = .;
    } > SRAM

    .data : AT(ADDR(.rodata) + SIZEOF(.rodata)) {
        __data_start = .;
        *(.data .data.*);
        __data_end = .;
    } > SRAM

    __data_load_start = LOADADDR(.data);

    /DISCARD/ : {
        *(.ARM.exidx);
    }
}