{
  'clientsettings.json': {},
  'initializationOptions.json': {},
  'settings.json': import './settings.json',
  'settings.schema.json': import './settings.schema.json',
  'template.json': {
    name: 'jsonnet',
    programArgs: {
      default: 'grustonnet-ls',
    },
    fileTypeMappings: [
      {
        fileType: {
          patterns: [
            '*.jsonnet',
          ],
        },
        languageId: 'jsonnet',
      },
      {
        fileType: {
          patterns: [
            '*.libsonnet',
          ],
        },
        languageId: 'jsonnet',
      },
    ],
  },
}
