SECTIONS
{
. = 0x100b0;
.text : { *(.text) }
. = 0x11000;
.data : { *(.data) }
}
