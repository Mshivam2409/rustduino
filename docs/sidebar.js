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
      items: ['rust/index_rust', 'rust/rust', 'rust/why-rust', 'rust/unsafe']
    },
    {
      type: 'category',
      label: 'Embedded',
      items: [
        'embedded/index_embedded',
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
        'core/index_core',
        'core/interrupts',
        'core/watchdog',
        'core/sleep_mode',
        'core/power',
        'core/port'
      ]
    },
    {
      type: 'category',
      label: 'Communication',
      items: ['com/index_com', 'com/i2c', 'com/usart']
    },
    {
      type: 'category',
      label: 'I/O Features',
      items: ['analog/index', 'analog/analog', 'analog/digital']
    },
    {
      type: 'category',
      label: 'Sensors',
      items: [
        'sensors/index_sensors',
        'sensors/aht10',
        'sensors/mpu6050',
        'sensors/servo'
      ]
    },
    {
      type: 'category',
      label: 'AVR Tools',
      items: ['tools/shift', 'tools/display']
    },
    {
      type: 'category',
      label: 'Math Library',
      items: ['math/index_math', 'math/map', 'math/random', 'math/micromath']
    },
    {
      type: 'category',
      label: 'Guides',
      items: [
        'guides/index_guide',
        'guides/blink',
        'guides/print',
        'guides/writing_string',
        'guides/analog_io',
        'guides/temp_sensor',
        'guides/motion',
        'guides/generate_analog',
        'guides/generate_mpu'
      ]
    },
    {
      type: 'doc',
      label: 'Contributors',
      id: 'contributors'
    }
  ]
}
