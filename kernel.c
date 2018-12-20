/*
* Copyright (C) 2014  Arjun Sreedharan
* License: GPL version 2 or higher http://www.gnu.org/licenses/gpl.html
*/


#define SPI0_CS 0x7E204000
#define SPIO_FIFO 0x7E204004
#define SPI0_CLK 0x7E204008

#define GPIO_SELECT_FUN0	0x7E200000	//setting GPIO's PIN in/out
#define GPIO_SELECT_FUN1	0x7E200004	//setting GPIO's PIN in/out
#define GPIO_SELECT_FUN2	0x7E200008	//setting GPIO's PIN in/out
#define GPIO_SET0	0x7E20001C
#define GPIO_CLR0 0x7E200028

#define AUXENB	0x7E215004 //ENABLING SPI's and UART

static unsigned int *lcd_clock;
static unsigned int *lcd_data;
static unsigned int *lcd_cs;



void init_spi()
{
	int *access = AUXENB;
	int *cs = SPI0_CS;
	int *clk = SPI0_CLK;
	int *gpio_select = GPIO_SELECT_FUN0;

	/*enable spi access*/
	*access &= 0x6;

	/****************************************** 
	 *	configure gpio's in/out 
	 *
	 * GPIO7  SPI0_CE1_N  P1-26  (use for reset)
	 * GPIO8  SPI0_CE0_N  P1-24
	 * GPIO9  SPI0_MISO   P1-21
	 * GPIO10 SPI0_MOSI   P1-19
	 * GPIO11 SPI0_SCLK   P1-23
	 * alt function 0 for all of the above
	 *
	 * P1 1  +3V3
	 * P1 25  GND
	 *
	 * P1 22 GPIO25  use as D/C
	 ******************************************/
	*gpio_select&=~(7<<27); //gpio9
	*gpio_select|=4<<27;    //alt0
	*gpio_select&=~(7<<24); //gpio8
	*gpio_select|=4<<24;    //alt0
	*gpio_select&=~(7<<21); //gpio7
	*gpio_select|=4<<21;    //alt0
	gpio_select=GPIO_SELECT_FUN1;
	*gpio_select&=~(7<<0); //gpio10/
	*gpio_select|=4<<0;    //alt0
	*gpio_select&=~(7<<3); //gpio11/
	*gpio_select|=4<<3;    //alt0
	gpio_select=GPIO_SELECT_FUN2;
	*gpio_select&=~(7<<15); //gpio25/
	*gpio_select|=1<<15;    //output

	/* configure SPI interface to be used by LCD*/
	// configure CS
	*cs &= 0x30; //CPHA=0, CPOL=0, TX and RX FIFO's cleared;

	// set clock
	*clk &= 0x1A;
}

void spi_one_byte ( unsigned int value )
{
	int *spi0_cs = SPI0_CS;
	int *spio_fifo =
	// clear fifo and set TA = 1
	*spi0_cs |= 0x000000B0;
	while(1)
	{
		// if TX_FIFO can accept one byte,
		if(*spi0_cs & (1<<18)) 
			break; //TXD
	}

	*spi0_cs |= value & 0xFF;
	while(1) { 
		// if transfer is DONE,
		if(*spi0_cs & (1<<16)) 
			break;
	}
	// TA = 0
	*spi0_cs |= 0x00000000; //cs0 comes back up
}

void write_spi_command(unsigned int cmd)
{
	int *clr = GPIO_CLR0;
	*clr |= 1<<25;
	spi_one_byte(cmd);

}

void write_spi_data(unsigned int data)
{
	int *set = GPIO_SET0;
	*set |= 1<<25;
	spi_one_byte(cmd);


}

void SetPageStart ( unsigned int x )
{
	spi_command(0xB0|(x&0x07));
}
//------------------------------------------------------------------------
void SetColumn ( unsigned int x )
{
	x+=0x20;
	spi_command(0x10|((x>>4)&0x0F));
	spi_command(0x00|((x>>0)&0x0F));
}


void read_spi_data()
{
}

void write_screen()
{

}

void kmain(void)
{
	const char *str = "my first kernel";
	unsigned int i = 0;
	unsigned int j = 0;
	unsigned int screensize;

	/* this loops clears the screen
	* there are 25 lines each of 80 columns; each element takes 2 bytes */
	init_spi();
	while (1) {

	}

	return;
}

