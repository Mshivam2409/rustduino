# Introduction 

The ATmega 2560p features a 10-bit Analog to digital converter.The ADC being 
connected to 8/26-channel Analog mltiplexer allow 8/16 single ended voltage inputs.

The device also supports 16/32 differential voltage input combinations.Four of which
are equipped with programmable gain stage, providing amplification steps of 0 dB, 20 
dB, 46 dB on the differential input voltage before the ADC conversion.The 16 channels 
are split into two sections of 8 in each of which 7 differential analog input channel 
share a common negative terminal, while any other ADC input in that section can be 
selected as the positive input terminal.

The ADC contains a sample and hold circuit which ensures that the input volatge to
the ADC is held at a constant level during conversion, it also has a seperate analog 
supply pin, AVCC.Internal Reference voltages of nominally 1.1V, 2.56V or AVCC are 
provided On-chip.

**Note** that the Power Reduction ADC bit, PRADC, must be disabled to enable ADC.

## Operation

The ADC converts the voltage on the analog input to 10 bit digital value by 
approximation, where minimum value is GND and maximum is voltage on AREF pin minus 1
LSB. The internal reference voltage may be connected by writing to the REFSn bits in
ADMUX Register. 

The analog input channel is selected by writing to the MUX bits in ADMUX and ADCSRB. 
Any of the ADC input pins, as well as GND and a fixed bandgap voltage reference, can
be selected as single ended inputs to the ADC. If differential channels are selected,
the voltage difference between the selected input channel pair then becomes
the analog input to the ADC.


