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
      items: ['arduino/index', 'arduino/atmega328p', 'arduino/atmega4809p']
    },
    {
      type: 'category',
      label: 'Core Concepts',
      items: ['core/watchdog', 'core/power', 'core/ports', 'core/gpio']
    },
    {
      type: 'category',
      label: 'Guides',
      items: ['guides/blink']
    },
    {
      type: 'doc',
      label: 'Contributors',
      id: 'contributors'
    },
    {
      type:'communication',
      label:'I2C',
      items:['sensors/aht10','com/i2c']
    },
    
  ]
}
