import rollup_nre from '@rollup/plugin-node-resolve';
import rollup_tsc from '@rollup/plugin-typescript';
// import { terser } from 'rollup-plugin-terser';

export default [
  {
    input: './src-ui/src/main.ts',
    output: {
      file: './dist/js/app-bundle.js',
      format: 'iife',
      name: 'bundle',
      sourcemap: true
    },
    plugins: [
      // rollup_cjs(),
      rollup_nre(),
      rollup_tsc({
        tsconfig: './src-ui/tsconfig.json',
        compilerOptions: {
          declaration: false,
          declarationDir: null
        }
      }),
      // terser({ compress: true, format: { comments: false } })
    ]
  }
]

