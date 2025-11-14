{
  name: 'grustonnet',
  displayName: 'Grustonnet Language Server',
  description: 'Full code support (formatting, highlighting, navigation, debugging etc) for Jsonnet',
  license: 'Apache License Version 2.0',
  publisher: 'ppi',
  version: '0.1.0',
  repository: {
    type: 'git',
    url: '',
  },
  engines: {
    vscode: '^1.61.0',
  },
  categories: [
    'Programming Languages',
    'Linters',
    'Formatters',
    'Debuggers',
  ],
  keywords: [
    'jsonnet',
    'lsp',
    'language',
    'debugger',
  ],
  activationEvents: [
    'onLanguage:jsonnet',
  ],
  main: './out/extension',
  contributes: {
    menus: {
      'editor/title': [
        {
          command: 'jsonnet.evalFile2',
          alt: 'jsonnet.evalFileYaml',
          group: 'navigation',
        },
      ],
      'editor/title/run': [
        {
          command: 'jsonnet.debugEditorContents',
          when: 'resourceLangId == jsonnet',
          group: 'navigation@1',
        },
      ],
    },
    commands: [
      {
        command: 'jsonnet.evalFile2',
        title: 'Jsonnet: Evaluate File',
        enablement: 'resourceLangId == jsonnet',
        icon: '$(open-preview)',
      },
      {
        command: 'jsonnet.restartLanguageServer',
        title: 'Jsonnet: Restart Language Server',
      },
      {
        command: 'jsonnet.debugEditorContents',
        title: 'Jsonnet: Debug File',
        category: 'Jsonnet',
        enablement: '!inDebugMode',
        icon: '$(debug-alt)',
      },
    ],
    languages: [
      {
        id: 'jsonnet',
        aliases: [
          'Jsonnet',
          'jsonnet',
        ],
        extensions: [
          '.jsonnet',
          '.libsonnet',
        ],
        configuration: './language/configuration.jsonc',
      },
    ],
    grammars: [
      {
        language: 'jsonnet',
        scopeName: 'source.jsonnet',
        path: './language/jsonnet.tmLanguage.json',
      },
    ],
    breakpoints: [
      {
        language: 'jsonnet',
      },
    ],
    debuggers: [
      {
        type: 'jsonnet',
        languages: [
          'jsonnet',
        ],
        label: 'Jsonnet Debugger',
        configurationAttributes: {
          launch: {
            required: [
              'program',
              'jpaths',
            ],
            properties: {
              program: {
                type: 'string',
                description: 'jsonnet script to run',
              },
              jpaths: {
                type: 'array',
                description: 'jsonnet search paths',
                items: {
                  type: 'string',
                },
              },
            },
          },
        },
        initialConfigurations: [],
        configurationSnippets: [
          {
            label: 'Jsonnet: Debug current file',
            description: 'A new configuration for debugging a Jsonnet file.',
            body: {
              type: 'jsonnet',
              request: 'launch',
              name: 'Debug current JSONNET file',
              program: '^"\\${file}"',
            },
          },
        ],
      },
    ],
    local schema = import 'schema.json',
    configuration:
      [
        {
          title: field.key,
          //orig: field.value,
        } + field.value {
          properties: {
            local defaultValue = std.get(std.get(field.value, 'default', null), key.key, null),
            ['jsonnet.languageServer.config.' + field.key + '.' + key.key]: key.value {
              [if defaultValue != null then 'default']: defaultValue,
            }
            for key in std.objectKeysValues(std.get(field.value, 'properties', default=field.value))
          },
        }
        for field in std.objectKeysValues(schema.properties)
      ] +
      [
        {
          type: 'object',
          title: 'Grustonnet Language Server',
          properties: {
            'jsonnet.languageServer.pathToBinary': {
              scope: 'resource',
              type: 'string',
              description: 'Path to language server binary',
              default: 'grustonnet-ls',
            },
            'jsonnet.languageServer.continuousEval': {
              scope: 'resource',
              type: 'boolean',
              default: true,
              description: 'Whether to continuously evaluate the selected file',
            },
            'jsonnet.debugger.releaseRepository': {
              type: 'string',
              default: 'grafana/jsonnet-debugger',
              description: 'Github repository to download the debugger server from',
            },
            'jsonnet.debugger.enableAutoUpdate': {
              scope: 'resource',
              type: 'boolean',
              default: true,
            },
            'jsonnet.debugger.pathToBinary': {
              scope: 'resource',
              type: 'string',
              description: 'Path to debugger',
            },
          },
        },
      ],
  },
  dependencies: {
    '@types/vscode': '^1.69.0',
    'vscode-languageclient': '^8.0.0',
    yaml: '^1.10.2',
  },
  devDependencies: {
    '@types/node': '^12.12.0',
    '@typescript-eslint/eslint-plugin': '^4.23.0',
    '@typescript-eslint/parser': '^4.23.0',
    '@vscode/vsce': '^3.7.0',
    eslint: '^7.26.0',
    typescript: '^4.4.3',
  },
  scripts: {
    'vscode:prepublish': 'npm run compile',
    compile: 'tsc -b',
    watch: 'tsc -b -w',
    lint: 'eslint ./src --ext .ts,.tsx',
  },
}
