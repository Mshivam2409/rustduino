## AVR - ATMEGA 2560 P Control HAL 
The folder HAL contains the Rust code for Hardware Layer Abstraction of AVR chip.
We have disabled WatchDog, enabled Clock Gating.
Then we Created GPIO pins for input/output control and then combined everything inside the main for LED blinking.





## Analog 

### Features

### Operation

### Starting a Conversion

A single Conversion is Started by writing a logical one to the ADC Start Conversion
bit,ADSC. This bit stays high as long as the conversion is in progress and will be
cleared by hardware when it is finished. While a conversion is in progress and
channel is changed ADC will finish the conversion in progress before changing channel.

Conversion can be enabled by various sources, which can be selected through ADC
Trigger Select bits, ADTS in ADCSRB.If a trigger signal is set still when a
conversion is done, the next positive edge will be ignored and new conversion won't
be started.

**Note** that the Interrupt Flag will be set even if the specific interrupt is
disabled or the Global Interrupt Enable bit in SREG is cleared. A conversion can take
place without causing an interrupt. However, the Interrupt Flag must be cleared in
order to trigger a new conversion at the next interrupt event.

Using the ADC Interrupt Flag as a trigger source makes the ADC start a new conversion
as soon as the ongoing conversion has finished and the ADC runs in the Free Running
mode, constantly sampling and updating ADC Data register. The first conversion must
be started by writing a logical one to the ADSC bit in ADCSRA.

Using the ADC Interrupt Flag as a Trigger source makes the ADC start a new conversion
as soon as the ongoing conversion has finished. The ADC then operates in Free Running
mode, constantly sampling and updating ADC Data Register, the first must be started
by writing logical one to the ADSC bit. In this mode ADC performs succesive
conversions independent of whether the ADC Interrupt Flag, ADIF is cleared or not.
If Auto Triggering is enabled, single conversions can be started by writing ADSC in 
DCSRA to one. ADSC can also be used to determine if a conversion is in progress.

### Prescaling and Conversion Timing

By default, the approximation circuitry requires an analog input clock frequency 
between 50 to 200 khz,which may reach as high as 1000khz in case of resolution less 
than 10 bits.

The ADC contains a prescaler which generates an acceptable ADC clock Frequency from 
any CPU frequency above 100khz, it is set by ADPS bits in ADCSRA.It continously 
counts from moment ADC is switched on and is continously reset when ADEN is Low.

When initiating single ended conversion, the conversion starts at the following
rising edge of the ADC clock cycle(what is ADC clock cycle rising edge?). A normal
conversion takes 13 ADC clock cycles, while the first takes 25 clock cycles to
initialize the analog circuitry.

When Bandgap reference voltage is used as input to the ADC, it will take a certain
time to stabilize. The actual sample and hold takes place 1.5 ADC clock cycles after
the start of a normal conversion and 13.5 ADC clock cycles after the 
**First Conversion**. When a conversion is complete, result is written to the Data 
registers, and ADIF is set.In single conversion, ADSC is cleared simultaneously.

#### Differential Channels

Differential conversions are synchronized to the internal clock equal to the half of
the ADC clock. This synchronization is done automatically by the ADC interface such
that the sample and hold occurs at a specific phase of the internal clock. A
**conversion initiated by the user** when internal clock is low will take same amount
of time as a single ended conversion(13 ADC clock cycles from the next prescaled
clock cycle) while if the internal clock is high then it will take 14 ADC clock
cycles due to synchronization mechanism. In Free running mode, since internal
clock is high at this time, all automatically started(that is all, but the first)
Free running conversions will take 14 ADC clock cycles.

If Auto Triggering is used ADC prescaler is reset before the conversion is started.
Since the stage is dependent of a stable ADC clock prior to the conversion, this
conversion will not be valid. By disabling nad enabling ADC between each conversion, 
only extended conversions are performed. The result from the extended conversions
will be valid.

### Changing Channel or Reference Selection

The MUXn and REFS bits in the ADMUX Register are single buffered through a temporary
register to which the CPU has random access to ensure the channels and reference
selection only takes place at a safe point during the conversion. The channel and
reference selection is continously updated until conversion Starts, then it stops or
locks to ensure sufficient sampling time for the ADC.

Continous updating resumes in the last ADC clock cycle before the conversion complete
(ADIF in ADCSRA is set). So do not write new channel or reference until one ADC clock
cycle after ADSC is written. 

If Auto Triggering is used, the exact time of the triggering event can be
interdeterministic. So take special care of when to update ADMUX as to control which
conversion is going to be affected by the new settings.

If both ADATE and ADEN are written, it cannot be predicted next conversion is based
on which settings, old or new. So **to safely update ADMUX** be sure that ADATE and
ADEN are both cleared, If During conversion then after minimum one ADC clock cycle
and if after a conversion the before the Interrupt flag (used as trigger source) is 
cleared.

If differential channel is used stage may take as much as 125 microseconds to 
stabilize to the new value so either no conversions should be started or conversions
should be discarded during this period. Same settling time should be observed for the differential conversion after changing ADC reference.

#### ADC Input Channels





### ADC Noise Canceler



### ADC Conversion Result