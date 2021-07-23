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
      items: ['core/watchdog', 'core/power', 'core/ports', 'core/gpio']
    },
    {
      type:'category',
      label:'Hardware Abstraction Layer',
      items: ['atmega2560p/hal/hal']
    },
    {
      type:'category',
      label:'Communications',
      items:['atmega2560p/com/usart','atmega2560p/com/i2c']
    },
    {
      type:'category',
      label:'Analog',
      items:['atmega2560p/analog/analog']
    },
    {
      type:'category',
      label:'Sensors',
      items:['sensors/mpu6050']
    },
    {
      type: 'category',
      label: 'Guides',
      items: ['guides/blink']
    },
    {
      type: 'category',
      label: 'Contributors',
      id: 'contributors'
    }
  ]
}
