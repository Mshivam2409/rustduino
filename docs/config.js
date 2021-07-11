module.exports = {
  projectName: 'Rust Duino',
  projectSlug: 'rustduino',
  projectTagLine: 'Electronics Club Summer Project',
  updateTags: [
    {
      image: 'oryd/keto',
      files: ['docs/docs/configure-deploy.md']
    }
  ],
  updateConfig: {
    src: '.schema/config.schema.json',
    dst: './docs/docs/reference/configuration.md'
  }
}
