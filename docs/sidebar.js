// module.exports = {
//   Introduction: ['index', 'install'],

//   Embedded: [
//     'embedded/index',
//     'embedded/registers',
//     'embedded/memory-mapped-io',
//     'embedded/data-protocols'
//   ]
// }

module.exports = {
  mySidebar: [
    {
      type: 'category',
      label: 'Introduction',
      items: ['index', 'install']
    },
    {
      type: 'category',
      label: 'Rust',
      items: ['rust/rust', 'rust/why-rust', 'rust/unsafe']
    },
    {
      type: 'category',
      label: 'Embedded',
      items: [
        'embedded/index',
        'embedded/registers',
        'embedded/memory-mapped-io',
        'embedded/data-protocols'
      ]
    },
    {
      type: 'category',
      label: 'Arduino',
      items: ['arduino/index', 'arduino/atmega328p', 'arduino/atmega2560p']
    },
    {
      type: 'category',
      label: 'Core Concepts',
      items: [
        'core/watchdog',
        'core/interrupts', 
        'core/power', 
        'core/port', 
        'core/gpio', 
        'core/sleep_mode'
      ]
    },
    {
      type:'category',
      label:'Communication',
      items:['com/i2c', 'com/usart']
    },
    {
      type: 'category',
      label: 'Analog',
      items: ['analog/analog', 'analog/digital']
    },
    {
      type: 'category',
      label: 'AVR Tools',
      items: ['avr/shift','avr/display']
    },
    {
      type: 'category',
      label: 'Sensors',
      items: ['sensors/aht10','sensors/mpu6050']
    },
    {
      type: 'category',
      label: 'Math Library',
      items: ['math/index_math','math/map','math/random']
    },
    {
      type: 'category',
      label: 'Guides',
      items: [
        'guides/blink',
        'guides/print',
        'guides/writing_string',
        'guides/analog_io',
	      'guides/temp_sensor'
      ]
    },
    {
      type: 'doc',
      label: 'Contributors',
      id: 'contributors'
    }
    
  ]
}
