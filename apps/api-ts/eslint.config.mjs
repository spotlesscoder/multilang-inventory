import tsParser from '@typescript-eslint/parser';
import baseConfig from '../../eslint.config.mjs';

export default [
    ...baseConfig,
    {
        ignores: ['webpack.config.js'],
    },
    {
        files: ['**/*.ts', '**/*.tsx', '**/*.js', '**/*.jsx'],
        languageOptions: {
            parser: tsParser,
            parserOptions: {
                project: ['apps/api/tsconfig.*?.json'],
            },
        },
        rules: {
            'no-underscore-dangle': [
                'error',
                {
                    allow: ['_id'],
                },
            ],
        },
    },
];
