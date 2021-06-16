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
    }
  ]
}
