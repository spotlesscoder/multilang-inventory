import nx from '@nx/eslint-plugin';

export default [
    ...nx.configs['flat/base'],
    ...nx.configs['flat/typescript'],
    ...nx.configs['flat/javascript'],
    {
        ignores: ['**/dist'],
    },
    {
        files: ['**/*.ts', '**/*.tsx', '**/*.js', '**/*.jsx'],
        rules: {
            '@nx/enforce-module-boundaries': [
                'error',
                {
                    enforceBuildableLibDependency: true,
                    allow: ['^.*/eslint(\\.base)?\\.config\\.[cm]?js$'],
                    depConstraints: [
                        {
                            sourceTag: '*',
                            onlyDependOnLibsWithTags: ['*'],
                        },
                    ],
                },
            ],
            '@typescript-eslint/await-thenable': 'error',
            '@typescript-eslint/explicit-function-return-type': 'error',
            '@typescript-eslint/explicit-module-boundary-types': 'off',
            '@typescript-eslint/interface-name-prefix': 'off',
            '@typescript-eslint/no-floating-promises': 'error',
            '@typescript-eslint/no-implied-eval': 'error',
            '@typescript-eslint/no-non-null-assertion': 'off',
            '@typescript-eslint/no-unnecessary-condition': [
                'off',
                { allowConstantLoopConditions: true },
            ],
            '@typescript-eslint/no-explicit-any': 'error',
            '@typescript-eslint/no-unused-vars': [
                'error',
                { argsIgnorePattern: '^_' },
            ],
            'class-methods-use-this': 'off',
            'consistent-return': 'error',
            curly: ['error', 'all'],
            eqeqeq: ['error', 'always'],
            'func-names': 'error',
            'init-declarations': 'off',
            'linebreak-style': 'error',
            'max-len': [
                'warn',
                {
                    code: 120,
                    ignoreComments: true,
                    ignorePattern: '^import .*',
                    ignoreStrings: true,
                    ignoreTemplateLiterals: true,
                },
            ],
            'no-await-in-loop': 'error',
            'no-console': 'error',
            'no-else-return': 'error',
            'no-empty-function': 'error',
            'no-eval': 'error',
            'no-implicit-globals': 'error',
            'no-implied-eval': 'error',
            'no-param-reassign': 'error',
            'no-plusplus': 'error',
            'no-underscore-dangle': 'error',
            'no-var': 'error',
            'prefer-template': 'error',
        },
    },
    {
        files: ['**/*.spec.js', '**/*.spec.ts'],
        plugins: {
            jest: require('eslint-plugin-jest'),
        },
        rules: {
            'jest/no-disabled-tests': 'off',
            'jest/no-focused-tests': 'error',
            'jest/no-identical-title': 'error',
            'jest/prefer-to-have-length': 'off',
        },
    },
    {
        files: ['**/*.cy.ts'],
        plugins: {
            jest: require('eslint-plugin-jest'),
        },
        rules: {
            'jest/valid-expect': 'off',
        },
    },
];
